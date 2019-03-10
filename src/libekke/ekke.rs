use std::
{
	  env
	, path::PathBuf
	, sync::Arc
	, rc::Rc
	, cell::RefCell
	, convert::TryFrom
};

use actix             :: { prelude::*, registry::SystemService                          } ;
use clap              :: { App as AppCli, Arg, ArgMatches, crate_version, crate_authors } ;
use failure           :: { ResultExt as _                                               } ;
use futures           :: { future::ok                                                   } ;
use futures_util      :: { future::{ FutureExt, join_all }, try_future::TryFutureExt    } ;
use hashbrown         :: { HashMap                                                      } ;
use parking_lot       :: { RwLock                                                       } ;
use slog              :: { Logger, Drain, debug, info, error, o                         } ;
use slog_term         :: { TermDecorator, CompactFormat                                 } ;
use slog_async        :: { Async                                                        } ;
use slog_unwraps      :: { ResultExt as _                                               } ;
use typename          :: { TypeName                                                     } ;

use tokio_async_await :: { await                                                        } ;
use tokio_uds         :: { UnixStream                                                   } ;

use ekke_io::
{
	IpcPeer               ,
	RegisterServiceMethod ,
	Rpc                   ,
	ThreadLocalDrain      ,
};

use ekke_config :: { Config                    } ;
use crate       :: { Settings, /*EkkeServer,*/ App } ;




mod     register_application   ;
pub use register_application::*;

mod     rpc_address   ;
pub use rpc_address::*;

#[ derive( TypeName ) ]
//
pub struct Ekke
{
	pub log : Logger,
	settings: Arc<RwLock< Config<Settings> >>,
	rpc     : Addr< Rpc >                           ,
	apps    : Rc < RefCell< HashMap<String, App> >> ,
	// http    : Rc < RefCell< EkkeServer           >> ,
}



impl Default for Ekke
{
	fn default() -> Self
	{
		let log = Self::root_logger().new( o!( "thread_name" => "main", "Actor" => "Ekke" ) );
		let rpc      = Rpc::new( log.new( o!( "Actor" => "Rpc" ) ), crate::service_map ).start();

		debug!( &log, "Trying to read default config file" );

		let defaults = env::current_exe().unwraps( &log ).parent().unwrap().join( "../../ekke_core/defaults.yml" );

		// let _serv_log = log.new( o!( "Actor" => "EkkeServer" ) );

		Ekke
		{
			settings: Arc::new( RwLock ::new(     Config::try_from( &defaults             ).unwraps( &log ) )),
			apps    : Rc ::new( RefCell::new(    HashMap::new     (                       )                 )),
			// http    : Rc ::new( RefCell::new( EkkeServer::new     ( serv_log, rpc.clone() )                 )),
			log     ,
			rpc     ,
		}
	}
}



impl Supervised    for Ekke {}
impl Actor         for Ekke { type Context = Context<Self>; }

impl SystemService for Ekke
{
	fn service_started( &mut self, ctx: &mut Context<Self> )
	{
		// println!( "{:#?}", *self.settings.read() );

		let log  = self.log.new( o!( "Actor" => "Ekke async block" ) );
		let rpc  = self.rpc .clone();
		// let _http = self.http.clone();
		let apps = self.apps.clone();
		// Register our services
		//
		self.register_service::<RegisterApplication>( &rpc, ctx );


		// Launch an ipc peer for each child application
		//
		let appcfgs = { self.settings.read().get().apps.clone() };

		let program = async move
		{
			info!( log, "Ekke Starting up" );

			// Launch each application. I think the implementation here is ok, but it's difficult
			// to prove that there can't be any synchronisation problems.
			//
			// We only get a peer after a connection exists, and as soon as a connection exists,
			// child apps send RegisterApplication. Our handler of RegisterApplication needs to have
			// the app object that we store on self, which only exists after we have a peer.
			//
			// In principle, as soon as a connection is incoming, and we have a peer, we store it in
			// self.apps, without any yield points between those 2 events. This means that normally
			// code for RegisterApplication can never run before we have saved the app object.
			//
			// However, another problem can arise. Our self.apps is in a refcell and we should not try
			// to borrow it twice. So here again, we guarantee that the borrow is in a critical?
			// section. That means there is no yield point...
			//
			await!( join_all( appcfgs.into_iter().map( |appcfg|
			{
				let l         = log.new( o!( "fn" => "App::launch" ) );
				let self_apps = apps.clone()                          ;

				App::launch( l, rpc.clone(), appcfg )

					.and_then( move |app|
					{
						self_apps.borrow_mut().insert( app.name.clone(), app );
						ok(())
					})

					.map_err( |err|
					{
						error!( log, "Cannot launch application: {}", err );
					})

			})));

			// keep the borrow as short as possible.
			// probably we should use a locking mechanism to prevent this being locked twice,
			// then as long as we are single threaded, this should not happen, as there is no
			// yield point (no awaits) within this block.
			//
/*			{
				http.borrow().run();
			}*/

			Ok(())
		};

		Arbiter::spawn( program.boxed().compat() );
	}
}

impl Ekke
{
	fn root_logger() -> Logger
	{
		let decorator = TermDecorator ::new().stdout()  .build()        ;
		let compact   = CompactFormat ::new( decorator ).build().fuse() ;
		let drain     = Async         ::new( compact   ).build().fuse() ;

		Logger::root( ThreadLocalDrain{ drain }.fuse(), o!( "version" => "0.1" ) )
	}




	pub async fn server_peer( log: Logger, rpc: Addr< Rpc > ) -> Addr< IpcPeer<UnixStream> >
	{
		let args       = Self::app_args();

		let sock_addr  = "\x00".to_string() + args.value_of( "socket" ).unwrap();

		let connection = await!( UnixStream::connect( PathBuf::from( &sock_addr ) ) )

			.context( "Failed to connect to socket" ).unwraps( &log );


		let peer_log = log.new( o!( "Actor" => "IpcPeer" ) );

		IpcPeer::create( |ctx: &mut Context<IpcPeer<UnixStream>>|
		{
			IpcPeer::new( connection, rpc, ctx.address(), peer_log )
		})
	}


	pub fn app_args() -> ArgMatches< 'static >
	{
		AppCli::new( "ekke_app" )

			.author ( crate_authors!() )
			.version( crate_version!() )
			.about  ( "Systemd frontend for the Ekke Framework" )


			.arg
			(
				Arg::with_name( "socket"  )

					.help ( "the socket on which to connect" )
					.long ( "socket" )
					.required( true )
					.value_name( "socket" )
			)

		.get_matches()
	}
}




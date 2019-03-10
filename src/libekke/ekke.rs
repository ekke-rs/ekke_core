use std::
{
	  env
	, str
	, path::PathBuf
	, sync::Arc
	, convert::TryFrom
};

use actix             :: { prelude::*, registry::SystemService                            };
use clap              :: { App, Arg, ArgMatches, crate_version, crate_authors             };
use failure           :: { ResultExt as _                                                 };
use futures_util      :: { future::FutureExt, try_future::TryFutureExt  };
use parking_lot       :: { RwLock                                                         };
use slog              :: { Logger, Drain, debug, info, o                                  };
use slog_term         :: { TermDecorator, CompactFormat                                   };
use slog_async        :: { Async                                                          };
use slog_unwraps      :: { ResultExt as _                                                 };
use typename          :: { TypeName                                                       };

use tokio_async_await :: { await, stream::StreamExt                                       };
use tokio_uds         :: { UnixStream, UnixListener                                       };

use ekke_io::
{
	IpcMessage            ,
	IpcPeer ,
	Rpc                   ,
	ThreadLocalDrain      ,
};

use ekke_config::{ Config };
use crate      ::{ EkkeError, Settings };



mod     register_application   ;
pub use register_application::*;

mod     rpc_address   ;
pub use rpc_address::*;

#[ derive( TypeName ) ]
//
pub struct Ekke
{
	log     : Logger                                ,
	rpc     : Addr< Rpc >                           ,
	settings: Arc< RwLock < Config<Settings>     >> ,
}



impl Default for Ekke
{
	fn default() -> Self
	{
		let log = Self::root_logger().new( o!( "thread_name" => "main", "Actor" => "Ekke" ) );
		let rpc      = Rpc::new( log.new( o!( "Actor" => "Rpc" ) ), crate::service_map ).start();

		debug!( &log, "Trying to read default config file" );

		let defaults = env::current_exe().unwraps( &log ).parent().unwrap().join( "../../ekke_core/defaults.yml" );

		Ekke
		{
			settings: Arc::new( RwLock ::new(     Config::try_from( &defaults             ).unwraps( &log ) )),
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

		let program = async move
		{
			info!( log, "Ekke Starting up" );

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

	pub async fn peer<'a>( sock_addr: String, rpc: Addr<Rpc>, log: &'a Logger ) -> Recipient< IpcMessage >
	{
		debug!( log, "Trying to bind to socket: {:?}", sock_addr );

		let connection = await!( Self::bind( &sock_addr ) ).context( "Failed to receive connections on socket" ).unwraps( log );
		let peer_log   = log.new( o!( "Actor" => "IpcPeer" ) );

		info!( log, "Listening on socket: {:?}", sock_addr );

		IpcPeer::create( |ctx: &mut Context<IpcPeer<UnixStream>>|
		{
			IpcPeer::new( connection, rpc, ctx.address(), peer_log )

		}).recipient()
	}


	// We only want one program to connect, so we stop listening after the first stream comes in
	//
	async fn bind<'a>( sock_addr: &'a str ) -> Result< UnixStream, failure::Error >
	{
		let     listener   = UnixListener::bind( sock_addr )?;
		let mut connection = listener.incoming();

		if let Some( income ) = await!( connection.next() )
		{
			// Return has to be here! We want to break from loop and function when we are connected.
			// We only allow one connection atm. It's not great security, but we only want our child
			// process to connect to us, so not allowing further connections.
			//
			// This does mean that if the connection would drop, child process cannot reconnect but needs to be
			// given a new socket, which currently is not implemented. That being said, on unix sockets, this
			// shouldn't be a problem in real life, but this is most certainly temporary code.
			//
			// TODO: Make secure ipc channel
			//
			return Ok( income? )
		};

		Err( EkkeError::NoConnectionsReceived )?
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
		App::new( "ekke_app" )

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




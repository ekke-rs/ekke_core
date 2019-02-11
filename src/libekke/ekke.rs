use std::
{
	  process::Command
	, str
};

use actix             :: { prelude::*                                        };
use failure           :: { ResultExt                                         };
use futures_util      :: { future::FutureExt, try_future::TryFutureExt, join };
use slog              :: { Logger, debug, info, o                            };

use tokio_async_await :: { await            , stream::StreamExt              };
use tokio_uds         :: { UnixStream       , UnixListener                   };

use ekke_io           :: { IpcPeer          , ResultExtSlog                  };
use crate::
{
	  Dispatcher
	, RegisterService
	, IpcHandler
	, EkkeError
};



mod register_application;
pub use register_application::*;


#[ derive( Debug, Clone ) ]
//
pub struct Ekke
{
	pub log: Logger
}

impl Actor for Ekke
{
	type Context = Context<Self>;

	// Start the server
	// Register our services with the dispatcher
	//
	fn started( &mut self, ctx: &mut Self::Context )
	{


		let our_address = ctx.address().clone();
		let log = self.log.clone();

		let program = async move
		{

			info!( log, "Ekke Starting up" );

			// panic!( "Everyting is on fire" );

			// TODO: use abstract socket
			//
			let address_b: &str = "ekke peer B";
			let address_c: &str = "ekke peer C";

			Command::new( "target/debug/ekke_systemd" )

				.arg( "--server" )
				.arg( address_b  )
				.spawn()
				.expect( "PeerA: failed to execute process" )
			;

/*			Command::new( "target/debug/peerc" )

				.arg( "--server" )
				.arg( address_c  )
				.spawn()
				.expect( "PeerA: failed to execute process" )
			;*/

			// We use abstract unix sockets.
			//
			let sock_addr_b = "\x00".to_string() + address_b;
			let sock_addr_c = "\x00".to_string() + address_c;

			let dispatcher = Dispatcher::new( log.new( o!( "Actor" => "Dispatcher" ) ) ).start();

			await!( dispatcher.send( RegisterService
			{
				  name   : "RegisterApplication".into()
				, service: IpcHandler::RegisterApplication( our_address )

			})).expect( "MailboxError" );


			println!( "Ekke: Starting IpcPeer" );


			let fb = Self::peer( &sock_addr_b, dispatcher.clone(), &log );
			let fc = Self::peer( &sock_addr_c, dispatcher.clone(), &log );


			#[allow(clippy::useless_let_if_seq)]
			//
			// TODO: check whether this should be filed as an issue against futures-preview
			//
			let ( _ipc_peerb, _ipc_peerc ) = join!( fb, fc );


			Ok(())
		};

		Arbiter::spawn( program.boxed().compat() );
	}
}




impl Ekke
{
	pub async fn peer<'a>( sock_addr: &'a str, dispatch: Addr<Dispatcher>, log: &'a Logger ) -> Addr< IpcPeer >
	{
		debug!( log, "Trying to bind to socket: {:?}", sock_addr );

		let connection = await!( Self::bind( sock_addr ) ).context( "Failed to receive connections on socket" ).unwraps( log );
		let peer_log   = log.new( o!( "Actor" => "IpcPeer" ) );

		info!( log, "Listening on socket: {:?}", sock_addr );

		IpcPeer::create( |ctx: &mut Context<IpcPeer>|
		{
			IpcPeer::new( connection, dispatch.recipient(), ctx.address(), peer_log )
		})

		// IpcPeer::new( connection, dispatch )
	}


	// We only want one program to connect, so we stop listening after the first stream comes in
	//
	async fn bind<'a>( sock_addr: &'a str ) -> Result< UnixStream, failure::Error >
	{
		let     listener   = UnixListener::bind( sock_addr )?;
		let mut connection = listener.incoming();

		while let Some( income ) = await!( connection.next() )
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
}




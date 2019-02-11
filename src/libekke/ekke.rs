use std::
{
	  process::Command
	, str
};

use actix             :: { prelude::*                                        };
use failure           :: { ResultExt                                         };
use futures_util      :: { future::FutureExt, try_future::TryFutureExt, join };
use slog              :: { Logger, info, o                                   };

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
			const SOCK_ADDRB: &str = "/home/user/peerAB.sock";
			const SOCK_ADDRC: &str = "/home/user/peerAC.sock";

			Command::new( "target/debug/ekke_systemd" )

				.arg( "--server" )
				.arg( SOCK_ADDRB  )
				.spawn()
				.expect( "PeerA: failed to execute process" )
			;


/*			Command::new( "target/debug/peerc" )

				.arg( "--server" )
				.arg( SOCK_ADDRC  )
				.spawn()
				.expect( "PeerA: failed to execute process" )
			;*/


			let dispatcher = Dispatcher::new( log.new( o!( "Actor" => "Dispatcher" ) ) ).start();

			await!( dispatcher.send( RegisterService
			{
				  name   : "RegisterApplication".into()
				, service: IpcHandler::RegisterApplication( our_address )

			})).expect( "MailboxError" );


			println!( "Ekke: Starting IpcPeer" );


			let fb = Self::peer( SOCK_ADDRB, dispatcher.clone(), &log );
			let fc = Self::peer( SOCK_ADDRC, dispatcher.clone(), &log );


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
		let connection = await!( Self::bind( sock_addr ) ).context( "Failed to receive connections on socket" ).unwraps( log );
		let peer_log   = log.new( o!( "Actor" => "IpcPeer" ) );

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
		let _ = std::fs::remove_file( sock_addr ); // .context( format!( "Cannot unlink socket address: {:?}", sock_addr ) )?;

		let listener = UnixListener::bind( sock_addr )?;
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




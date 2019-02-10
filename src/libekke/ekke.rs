use std::
{
	  process::Command
	, str
};

use actix             :: { prelude::*                                        };
use futures_util      :: { future::FutureExt, try_future::TryFutureExt, join };
use slog              :: { Logger, info, o                                   };

use tokio_async_await :: { await            , stream::StreamExt              };
use tokio_uds         :: { UnixStream       , UnixListener                   };

use ekke_io           :: { IpcPeer                                           };
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
		let connection = await!( Self::bind( sock_addr ) ).expect( "failed to bind socket address" );
		let peer_log   = log.new( o!( "Actor" => "IpcPeer" ) );

		IpcPeer::create( |ctx: &mut Context<IpcPeer>|
		{
			IpcPeer::new( connection, dispatch.recipient(), ctx.address(), peer_log )
		})

		// IpcPeer::new( connection, dispatch )
	}


	// We only want one program to connect, so we stop listening after the first stream comes in
	//
	async fn bind( sock_addr: &str ) -> Result< UnixStream, failure::Error >
	{
		let _ = std::fs::remove_file( sock_addr ); // .context( format!( "Cannot unlink socket address: {:?}", sock_addr ) )?;

		let listener = UnixListener::bind( sock_addr ).expect( "PeerA: Could not bind to socket" );
		let mut connection = listener.incoming();

		while let Some( income ) = await!( connection.next() )
		{
			match income
			{
				Ok ( stream ) => return Ok( stream ),
				Err( _ ) => { eprintln!( "PeerA: Got Invalid Stream" ); continue }
			};
		};

		Err( EkkeError::NoConnectionsReceived.into() )
	}
}




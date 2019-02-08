use tokio_uds::{ UnixStream, UnixListener };
use futures_util::{future::FutureExt, try_future::TryFutureExt};

use tokio_async_await::await;
use tokio_async_await::stream::StreamExt;

use actix::prelude::*;
use futures_util::{join};
use crate::
{
	  Dispatcher
	, RegisterService
	, Service
	, EkkeError
};

use ekke_io::{ IpcPeer, IpcMessage };

use std::
{
	  process::Command
	, str
	, collections::HashMap
};

use serde_derive::{Serialize, Deserialize};

#[ derive( Debug ) ]
//
pub struct Ekke
{

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


		let program = async move
		{
			println!( "Ekke: Starting up" );

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


			let dispatcher = Dispatcher { services: HashMap::new() }.start();

			await!( dispatcher.send( RegisterService
			{
				  name   : "RegisterApplication".into()
				, service: Service::RegisterApplication( our_address )

			})).expect( "MailboxError" );


			println!( "Ekke: Starting IpcPeer" );


			let fb = peer( SOCK_ADDRB, dispatcher.clone() );
			let fc = peer( SOCK_ADDRC, dispatcher.clone() );

			let ( _ipc_peerb, _ipc_peerc ) = join!( fb, fc );

			Ok(())

		};

		Arbiter::spawn( program.boxed().compat() );
	}
}





pub async fn peer( sock_addr: &str, dispatch: Addr<Dispatcher> ) -> Addr< IpcPeer >
{
	let connection = await!( bind( sock_addr ) ).expect( "failed to bind socket address");

	IpcPeer::create( |ctx: &mut Context<IpcPeer>| { IpcPeer::new( connection, dispatch.recipient(), ctx.address() ) } )

	// IpcPeer::new( connection, dispatch )
}


// We only want one program to connect, so we stop listening after the first stream comes in
//
async fn bind( sock_addr: &str ) -> Result< UnixStream, failure::Error >
{
	let _ = std::fs::remove_file( &sock_addr ); // .context( format!( "Cannot unlink socket address: {:?}", sock_addr ) )?;

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


#[ derive( Debug, Serialize, Deserialize ) ]
//
pub struct RegisterApplication
{
	pub app_name: String
}

impl Message for RegisterApplication
{
	type Result = IpcMessage;
}

impl Handler<RegisterApplication> for Ekke
{
	type Result = IpcMessage;

	fn handle( &mut self, msg: RegisterApplication, _ctx: &mut Context<Self> ) -> Self::Result
	{
		println!( "Ekke: Received app registration for app: {}", msg.app_name );

		IpcMessage{ service: "RegisterAppAck".into(), payload: "Thank you for registering with Ekke.".as_bytes().to_vec() }
	}

}


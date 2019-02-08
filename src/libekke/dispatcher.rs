use actix::prelude::*;
use std::collections::HashMap;
use crate::{ Ekke, services::RegisterApplication };
use ekke_io::{ IpcConnTrack, IpcMessage };
use serde_cbor::from_slice as des;
use futures_util::{future::FutureExt, try_future::TryFutureExt};
use tokio_async_await::await;


pub struct Dispatcher
{
	pub services: HashMap< String, Service >
}

impl Actor for Dispatcher { type Context = Context<Self>; }


impl Dispatcher
{
	fn service_no_handler( name: &str )
	{
		eprintln!( "No handler registered for service: {}", name );
	}

	fn error_response( error: String, addr: Recipient< IpcMessage > )
	{
		Arbiter::spawn( async move
		{
			await!( addr.send( IpcMessage::new( "EkkeServerError".into(), error ) ) ).expect( "MailboxError" );

			Ok(())

		}.boxed().compat() );
	}
}


impl Handler<IpcConnTrack> for Dispatcher
{
	type Result = ();


	#[allow(irrefutable_let_patterns)]

	fn handle( &mut self, msg: IpcConnTrack, _ctx: &mut Context<Self> ) -> Self::Result
	{
		match msg.ipc_msg.service.as_ref()
		{
			// RegisterApplication
			//
			name @ "RegisterApplication" =>
			{
				match self.services.get( name.into() )
				{
					Some( service ) =>
					{
						if let Service::RegisterApplication( addr ) = service
						{
							let de: RegisterApplication = des( &msg.ipc_msg.payload ).expect( "Failed to deserialize into RegisterApplication" );

							let addr = addr.clone();

							Arbiter::spawn( async move
							{
								let resp = await!( addr.send( de ) ).expect( "MailboxError" );

								await!( msg.ipc_peer.send( resp ) ).expect( "MailboxError" );

								Ok(())

							}.boxed().compat() )
						}
					},

					None => Self::service_no_handler( name )
				}
			}

			_ => Self::error_response( format!( "Ekke Server received request for unknown service: {:?}", &msg.ipc_msg.service ), msg.ipc_peer )
		}
	}
}


impl Handler<RegisterService> for Dispatcher
{
	type Result = ();


	fn handle( &mut self, msg: RegisterService, _ctx: &mut Context<Self> ) -> Self::Result
	{
		if let Some( service ) = self.services.get( &msg.name )
		{
			eprintln!( "Handler for service already registered: {}, ->{:?}", &msg.name, service );
		}

		else
		{
			self.services.insert( msg.name, msg.service );
		}
	}
}



#[ derive( Message ) ]
//
pub struct RegisterService
{
	pub name   : String,
	pub service: Service
}


#[ derive( Debug ) ]
//
pub enum Service
{
	RegisterApplication( Addr< Ekke > )
}

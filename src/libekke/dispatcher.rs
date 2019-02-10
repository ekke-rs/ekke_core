use std               :: collections::HashMap                               ;

use actix             :: prelude::*                                         ;
use futures_util      :: {future::FutureExt, try_future::TryFutureExt}      ;
use serde_cbor        :: from_slice as des                                  ;
use slog              :: Logger                                             ;
use tokio_async_await :: await                                              ;

use crate             :: { Ekke, services::RegisterApplication, EkkeError } ;
use ekke_io           :: { IpcConnTrack, IpcMessage }                       ;


#[macro_use]
mod dispatch_macros;
use request_response;


#[ derive( Debug, Clone ) ]
//
pub struct Dispatcher
{
	  handlers: HashMap< String, IpcHandler >
	, log     : Logger
}

impl Actor for Dispatcher { type Context = Context<Self>; }



impl Dispatcher
{
	pub fn new( log: Logger ) -> Self
	{
		Self { handlers: HashMap::new(), log }
	}


	/// Send an error message back to the peer application over the ipc channel.
	///
	fn error_response( error: String, addr: Recipient< IpcMessage > )
	{
		Arbiter::spawn( async move
		{
			await!( addr.send( IpcMessage::new( "EkkeServerError".into(), error ) ) ).expect( "MailboxError" );

			Ok(())

		}.boxed().compat() );
	}
}


/// Handle incoming IPC messages
///
impl Handler<IpcConnTrack> for Dispatcher
{
	type Result = ();


	/// Handle incoming IPC messages
	///
	/// Each type of incoming message the application accepts is listed here. We find the
	/// corresponding actor in our handlers list and forward the message.
	///
	/// Two macros are provided to get rid of the boiler plate.
	///
	/// `request_response!` will wait for the actor to send an IpcMessage back as response.
	/// `request_void!`     will forward and not wait for an answer.
	///
	fn handle( &mut self, msg: IpcConnTrack, _ctx: &mut Context<Self> ) -> Self::Result
	{
		match msg.ipc_msg.service.as_ref()
		{
			"RegisterApplication" => request_response!( self, msg, RegisterApplication ),

			_ => Self::error_response

					( format!( "Ekke Server received request for unknown service: {:?}", &msg.ipc_msg.service ), msg.ipc_peer )
		}
	}
}


/// We need to keep a list of service->actor handler mappings at runtime. This is where services
/// register.
///
impl Handler<RegisterService> for Dispatcher
{
	type Result = ();


	#[allow(clippy::suspicious_else_formatting)]
	//
	fn handle( &mut self, msg: RegisterService, _ctx: &mut Context<Self> ) -> Self::Result
	{
		if let Some( service ) = self.handlers.remove( &msg.name )
		{
			panic!( EkkeError::DoubleServiceRegistration( msg.name, service ) );
		}

		else
		{
			self.handlers.insert( msg.name, msg.service );
		}
	}
}



#[ derive( Debug, Clone ) ]
//
pub struct RegisterService
{
	pub name   : String,
	pub service: IpcHandler
}

impl Message for RegisterService { type Result = (); }


/// Map service names to actor address types.
///
#[ derive( Debug, Clone ) ]
//
pub enum IpcHandler
{
	RegisterApplication( Addr< Ekke > )
}


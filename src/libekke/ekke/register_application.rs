use actix             :: { prelude::*             };
use serde_derive      :: { Serialize, Deserialize };
use ekke_io           :: { IpcMessage             };

use crate::Ekke;

#[ derive( Debug, Clone, Serialize, Deserialize ) ]
//
pub struct RegisterApplication
{
	pub app_name: String
}

impl Message for RegisterApplication {	type Result = IpcMessage; }


impl Handler<RegisterApplication> for Ekke
{
	type Result = IpcMessage;

	fn handle( &mut self, msg: RegisterApplication, _ctx: &mut Context<Self> ) -> Self::Result
	{
		println!( "Ekke: Received app registration for app: {}", msg.app_name );

		IpcMessage{ service: "RegisterAppAck".into(), payload: b"Thank you for registering with Ekke.".to_vec() }
	}

}


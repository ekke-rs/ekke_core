use typename          :: { TypeName                        };
use actix             :: { prelude::*                      };
use serde_derive      :: { Serialize, Deserialize          };
use ekke_io           :: { IpcMessage, ConnID, MessageType };
use slog              :: { info                            };


use crate::Ekke;

#[ derive( Debug, Clone, Serialize, Deserialize, Message, TypeName ) ] #[ rtype( result="IpcMessage" ) ]
//
pub struct RegisterApplication
{
	pub conn_id : ConnID      ,
	pub app_name: String      ,
	pub route   : String      ,
	pub services: Vec<String> ,
}

#[ derive( Debug, Clone, Serialize, Deserialize, Message, TypeName ) ]
//
pub struct RegisterApplicationResponse
{
	pub response: String
}



impl Handler<RegisterApplication> for Ekke
{
	type Result = IpcMessage;

	fn handle( &mut self, msg: RegisterApplication, _ctx: &mut Context<Self> ) -> Self::Result
	{
		info!( self.log, "Ekke: Received app registration for app: {}", msg.app_name );

		let service = "RegisterApplicationResponse".to_string();
		let payload = RegisterApplicationResponse{ response: "Thank you for registering with Ekke".to_string() };
		let ms_type = MessageType::Response;
		let conn_id = msg.conn_id;

		IpcMessage::new
		(
			  service
			, payload
			, ms_type
			, conn_id
		)
	}
}


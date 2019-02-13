use typename          :: { TypeName               };
use actix             :: { prelude::*             };
use serde_derive      :: { Serialize, Deserialize };


use crate::Ekke;

#[ derive( Debug, Clone, Serialize, Deserialize, Message, TypeName ) ]
//
pub struct RegisterApplication
{
	pub app_name: String
}



impl Handler<RegisterApplication> for Ekke
{
	type Result = ();

	fn handle( &mut self, msg: RegisterApplication, _ctx: &mut Context<Self> ) -> Self::Result
	{
		println!( "Ekke: Received app registration for app: {}", msg.app_name );
	}

}


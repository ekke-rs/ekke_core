use crate :: { import::*, Ekke };


#[ derive( Debug, Clone, Serialize, Deserialize, Message, TypeName ) ] #[ rtype( result="IpcMessage" ) ]
//
pub struct RegisterApplication
{
	pub conn_id : ConnID      ,
	pub app_name: String      ,
	pub routes  : Vec<String> ,
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
		let conn_id = msg.conn_id.clone();

		// dbg!( self.apps.borrow().keys() );
		// TODO: Error handling
		// - apps need to be unique in terms of names and routes
		//
		{
			match self.apps.borrow_mut().get_mut( &msg.app_name )
			{
				Some( app ) =>
				{
					for route in &msg.routes
					{
						self.http.borrow_mut().add_route( route.to_string(), app.peer.clone() ).unwraps( &self.log );
					}

					app.register( msg );
				},

				None        =>
				{
					crit!( self.log, "Tried to store info about app that isn't available yet: {}", msg.app_name );
					unreachable!()
				},
			};
		}


		IpcMessage::new
		(
			  service
			, payload
			, ms_type
			, conn_id
		)
	}
}


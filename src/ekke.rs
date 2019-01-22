use actix::prelude::*;

use crate           ::          Result;
use crate::messages :: AppRegistration;

/// The main application object
///
pub struct Ekke
{

}


impl Actor for Ekke
{
	type Context = Context<Self>;
}


impl Handler< AppRegistration > for Ekke
{
	type Result = Result<()>;

	fn handle( &mut self, msg: AppRegistration, _: &mut Context<Self> ) -> Self::Result
	{
		println!("Registering application: {:?}", msg.name );
		println!("Description: {:?}", msg.description );

		Ok(())
	}
}


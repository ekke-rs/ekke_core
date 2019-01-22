use actix::Message;
use crate::Result;

/// Message type to register a new application
///
pub struct AppRegistration
{
	pub name       : String,
	pub description: String,
}

impl Message for AppRegistration
{
	type Result = Result<()>;
}

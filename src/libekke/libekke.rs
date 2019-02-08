//! This is the actual functionality for the ekke framework server. The binary contains just a very basic main function. All functionality is exposed through this library so you could build against it if needed.
//
#![ feature( await_macro, async_await, futures_api ) ]

mod ekke;
mod dispatcher;
mod errors;

pub use ekke::
{
	  Ekke
};


pub use dispatcher::
{
	  Dispatcher
	, RegisterService
	, Service
};

pub use errors::
{
	  EkkeError
	, EkkeResult
};

pub mod services
{
	pub use crate::ekke::RegisterApplication;
}

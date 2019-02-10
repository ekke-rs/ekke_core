//! This is the actual functionality for the ekke framework server. The binary contains just a very basic main function. All functionality is exposed through this library so you could build against it if needed.
//
#![ feature( await_macro, async_await, futures_api, nll, stmt_expr_attributes ) ]

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
	, IpcHandler
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

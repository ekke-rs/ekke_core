//! This is the actual functionality for the ekke framework server. The binary contains just a very basic main function. All functionality is exposed through this library so you could build against it if needed.
//
#![ feature( await_macro, async_await, futures_api, nll, stmt_expr_attributes, never_type ) ]

mod ekke;
mod errors;
mod config;


pub use ekke::
{
	Ekke
};

use config::
{
	Settings
};


pub use errors::
{
	  EkkeError
	, EkkeResult
};


pub mod services
{
	pub use crate::ekke::RegisterApplication;
	pub use crate::ekke::RegisterApplicationResponse;
}







use crate::services::*;
use ekke_io::{ IpcMessage, Rpc };
use actix::Recipient;


pub(crate) fn service_map( rpc: &Rpc, msg: IpcMessage, ipc_peer: Recipient< IpcMessage > )
{
    match msg.service.as_ref()
    {
        "RegisterApplication" => rpc.deser_into::<RegisterApplication>( msg, ipc_peer ),
        _ =>(),
    }
}

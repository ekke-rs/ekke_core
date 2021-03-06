#![ forbid(unsafe_code) ]
#![ feature( await_macro, async_await, futures_api, nll ) ]
#![ warn(unused_extern_crates) ]

use ekke_core  :: { Ekke                         };
use actix      :: { prelude::*                   };



// use log_panics ;



fn main()
{
	let sys = System::new( "Ekke Server" );

	let _app: Addr< Ekke > = SystemService::start_service( &Arbiter::new() );

	sys.run().expect( "Failed to run actix::System");
}








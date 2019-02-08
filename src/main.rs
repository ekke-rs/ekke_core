#![ feature( await_macro, async_await, futures_api ) ]

use libekke::Ekke;
use actix::prelude::*;


fn main()
{
	let sys = System::new( "peers" );

	let _ekke = Ekke{}.start();

	sys.run();
}

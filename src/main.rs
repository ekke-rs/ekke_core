use actix::prelude::*;
use futures::future::Future;

use libekke::*;
use messages::AppRegistration;



fn main()
{
		let system = System::new( "Ekke_Actix_System" );

		let ekke   = Ekke{}.start();
		let res    = ekke.send( AppRegistration{ name: "ekke_main".to_string(), description: "The main routine says hi!".to_string() } );

		Arbiter::spawn
		(
			res.map( |res|
			{
				match res
				{
					Ok (_) => println! ( "The application was successfully registered." ),
					Err(_) => eprintln!( "An Error occurred while trying to register the application." ),
				}
			})

				.map_err( |_| ())
		);

		system.run();
}

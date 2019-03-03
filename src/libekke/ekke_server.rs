use
{
	ekke_io::{ HttpServer, ResponseFuture },
	std::net::SocketAddr,

	hyper::{ Response, Request, Body },
	slog::{Logger, o},

};


pub struct EkkeServer
{
	_log: Logger,
	http: HttpServer
}


impl EkkeServer
{
	pub fn new( log: Logger ) -> Self
	{

		Self
		{
			http: HttpServer::new( log.new( o!( "Actor" => "HttpServer" ) ), Box::new( Self::responder ) ),
			_log: log,
		}
	}


	pub async fn run( &self )
	{
		let addr = SocketAddr::from( ([127, 0, 0, 1], 3000) );

		await!( self.http.run( addr ) );
	}


	fn responder( _req: Request<Body> ) -> ResponseFuture
	{
		Box::pin( async { Ok( Response::new( Body::from( "bla" ) ) ) } )
	}
}

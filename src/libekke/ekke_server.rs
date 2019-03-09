use
{
	actix        :: { Addr, Recipient, Arbiter                       },
	futures      :: { future::ok                                     },
	futures_util :: { future::FutureExt, try_future::TryFutureExt    },
	hashbrown    :: { HashMap                                        },
	hyper        :: { Response, Request, Body, StatusCode            },
	lazy_static  :: { lazy_static                                    },
	parking_lot  :: { Mutex                                          },
	slog         :: { Logger, o                                      },
	std          :: { net::SocketAddr, rc::Rc, sync::Arc             },
	typename     :: { TypeName                                       },
	tokio        :: { await },
	ekke_io      :: { HttpServer, ResponseFuture, Rpc, IpcMessage, SendRequest, MessageType, ConnID    },
	crate        :: { EkkeResult, EkkeError, FrontendRequest, BackendResponse                          },
};


// TODO: this will be consulted for every http request, try to do better than mutex
//       recipient is not sync, thus rwlock doesn't work.
//
lazy_static!
{
	static ref ROUTES: Arc< Mutex< HashMap< String, Recipient<IpcMessage> >>> = Arc::new( Mutex::new( HashMap::new() ));
}


#[ derive( Clone, TypeName ) ]
//
pub struct EkkeServer
{
	_log   : Logger        ,
	rpc    : Addr<Rpc>     ,
	http   : Rc< HttpServer >,
}





impl EkkeServer
{
	pub fn new( log: Logger, rpc: Addr<Rpc> ) -> Self
	{
		// TODO: build service_fn here, so we can put rpc in it withot sending that to ekke_io
		//
		let http = Rc::new( HttpServer::new( log.new( o!( "Actor" => "HttpServer" ) ), Box::new( Self::responder ), rpc.clone() ));

		Self
		{
			http     ,
			rpc      ,
			_log: log,
		}
	}


	pub fn add_route( &mut self, route: String, handler: Recipient< IpcMessage > ) -> EkkeResult<()>
	{
		match ROUTES.lock().insert( route.clone(), handler )
		{
			Some(_) => { Err( EkkeError::DoubleRouteRegistration( route ).into() ) },
			None    => { Ok(()) },
		}
	}


	pub fn run( &self )
	{
		let addr = SocketAddr::from( ([127, 0, 0, 1], 3000) );

		let http = self.http.clone();
		Arbiter::spawn( async move { await!( http.run( addr ) ) }.unit_error().boxed().compat() );
	}


	fn responder( req: Request<Body>, rpc: Addr<Rpc> ) -> ResponseFuture
	{
		// {
		// 	dbg!( ROUTES.lock().keys() );
		// }
		//
		// TODO: Just use string slice without last char if it's a slash
		//
		let mut p = req.uri().path().to_string();
		if p.ends_with( '/' ) { p.pop(); }

		let rpc = Arc::new( Mutex::new( rpc.recipient() ) );

		if let Some( ipc_peer ) = ROUTES.lock().get( &p )
		{
			let peer = ipc_peer.clone();

			let fut  = async move
			{
				let ipc_msg = IpcMessage::new
				(
					"FrontendRequest".to_string(),

					FrontendRequest
					{
						path: p,
						payload: Vec::new(),
					},

					MessageType::SendRequest,
					ConnID::new()
				);

				let response = await!( rpc.clone().lock().send
				(
					SendRequest
					{
						ipc_peer: peer,
						ipc_msg       ,
					}

				)).expect( "Send from ekkeserver to application" );

				let resp: BackendResponse = Rpc::deserialize( response.ipc_msg.payload ).expect( "failed to deserialize BackendResponse" );

				let body = Body::from( resp.body );

				Ok( Response::builder().status( StatusCode::from( resp.status ) ).body( body ).expect( "Cannot create hyper body" ) )
			};

			return Box::pin( fut )

			// Make a response
		}

		else
		{
			return Box::pin( ok( Response::builder().status( StatusCode::NOT_FOUND ).body( Body::from( "404" ) ).expect( "Cannot create hyper body" ) ) )

		}

		// Box::pin( async move
		// {
		//

		// 	let body = if let Some( ipc_peer ) = option
		// 	{
		// 		let ipc_msg = IpcMessage::new( p, "", MessageType::SendRequest, ConnID::new() );

		// 		let response = await!( rpc.clone().lock().send
		// 		(
		// 			SendRequest
		// 			{
		// 				ipc_peer: ipc_peer.lock().clone() ,
		// 				ipc_msg  ,
		// 			}

		// 		)).expect( "Send from ekkeserver to application" );

		// 		Body::from
		// 		(
		// 			"bla"
		// 		)
		// 	}

		// 	else
		// 	{
		// 		Body::from( "404" )
		// 	};

		// 	Ok( Response::builder().status( StatusCode::NOT_FOUND ).body( body ).expect( "Cannot create hyper body" ) )

		// } )
	}



}


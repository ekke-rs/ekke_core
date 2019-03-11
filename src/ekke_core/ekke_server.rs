use crate :: { import::*, FrontendRequest, BackendResponse, EkkeError, EkkeResult };


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


	fn responder( req: Request<Body>, rpc: Addr<Rpc>, log: Logger ) -> ResponseFuture
	{
		warn!( &log, "Start http processing: {}", req.uri(); "type" => "profile" );

		// We strip the trailing slash for route matching
		//
		let mut p = req.uri().path().to_string();
		if p.ends_with( '/' ) { p.pop(); }

		let rpc = Arc::new( Mutex::new( rpc.recipient() ) );


		// We have an app that registered this route. Send an IpcRequestOut to the peer.
		//
		if let Some( ipc_peer ) = ROUTES.lock().get( &p )
		{
			let peer = ipc_peer.clone();

			let fut = async move
			{
				let conn_id = ConnID::new();
				let ipc_msg = IpcMessage::new
				(
					"FrontendRequest".to_string(),

					FrontendRequest
					{
						path: p,
						payload: Vec::new(),
						conn_id: conn_id,
					},

					MessageType::IpcRequestOut,
					conn_id                   ,
				);

				let response = awaits!( rpc.clone().lock().send
				(
					IpcRequestOut{ ipc_peer: peer, ipc_msg }

				)).expect( "Send from ekkeserver to application" );


				// An ipc error might happen, -> internal server error, with error message as body
				//
				match response
				{
					// 200 - OK
					//
					Ok( r ) =>
					{
						let resp: BackendResponse = Rpc::deserialize( r.ipc_msg.payload ).expect( "failed to deserialize BackendResponse" );

						let body = Body::from( resp.body );

						warn!( &log, "Stop http processing: {}", req.uri(); "type" => "profile" );


						Ok( Response::builder().status( StatusCode::from( resp.status ) ).body( body ).expect( "Cannot create hyper body" ) )
					}

					// 501 - INTERNAL SERVER ERROR
					//
					Err( err ) =>
					{
						error!( log, "Internal Server Error: {}", &err );

						Ok
						(
							Response::builder().status( StatusCode::INTERNAL_SERVER_ERROR )

								.body  ( Body::from( format!("{}", &err) ) )
								.expect( "Cannot create hyper body"        )
						)
					}
				}


			};

			return Box::pin( fut )
		}

		// 404 - FILE NOT FOUND
		//
		else
		{
			info!( log, "404: [{}]", req.uri(); "type" => "access" );

			return Box::pin( ok
			(
				Response::builder()

					.status( StatusCode::NOT_FOUND      )
					.body  ( Body::from( "404" )        )
					.expect( "Cannot create hyper body" )
			))
		}
	}
}


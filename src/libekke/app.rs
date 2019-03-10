use
{
	actix             :: { Actor, Addr, AsyncContext, Context, Recipient         } ,
	failure           :: { ResultExt                                             } ,
	std               :: { fmt, path::PathBuf, process::Command                  } ,
	slog              :: { Logger, debug, info, o                                } ,
	slog_unwraps      :: { ResultExt as _                                        } ,

	tokio_uds         :: { UnixStream, UnixListener                              } ,
	tokio_async_await :: { await, stream::StreamExt                              } ,

	ekke_io           :: { ConnID, IpcMessage, IpcPeer, Rpc                      } ,
	crate             :: { AppConfig, EkkeResult, EkkeError, RegisterApplication } ,
};

mod frontend_request;
mod backend_response;

pub use frontend_request::*;
pub use backend_response::*;


pub struct App
{
	pub name     : String                  ,
	pub path     : PathBuf                 ,
	pub peer     : Recipient< IpcMessage > ,
	pub services : Vec<String>             ,
	pub routes   : Vec<String>             ,
}


impl App
{
	pub fn register( &mut self, r: RegisterApplication )
	{
		self.services = r.services;
		self.routes   = vec![ r.route ];
	}


	pub async fn launch( log: Logger, rpc: Addr< Rpc >, appcfg: AppConfig ) -> EkkeResult< Self >
	{
		dbg!( &appcfg );

		let addr      = ConnID::new().hex()        ;
		let sock_addr = "\x00".to_string() + &addr ;

		Command::new( &appcfg.path )

			.arg( "--socket" )
			.arg( &addr      )
			.spawn()?

		;

		// We use abstract unix sockets.
		//

		info!( log, "Starting IpcPeer for {}", &appcfg.name );

		let peer = await!( Self::peer( sock_addr, rpc, &log ) );

		let AppConfig{ name, path } = appcfg;

		Ok( Self
		{
			name                 ,
			path                 ,
			peer                 ,
			routes  : Vec::new() ,
			services: Vec::new() ,
		})
	}


	pub async fn peer<'a>( sock_addr: String, rpc: Addr<Rpc>, log: &'a Logger ) -> Recipient< IpcMessage >
	{
		debug!( log, "Trying to bind to socket: {:?}", sock_addr );

		let connection = await!( Self::bind( &sock_addr ) ).context( "Failed to receive connections on socket" ).unwraps( log );
		let peer_log   = log.new( o!( "Actor" => "IpcPeer" ) );

		info!( log, "Listening on socket: {:?}", sock_addr );

		IpcPeer::create( |ctx: &mut Context<IpcPeer<UnixStream>>|
		{
			IpcPeer::new( connection, rpc, ctx.address(), peer_log )

		}).recipient()
	}


	// We only want one program to connect, so we stop listening after the first stream comes in
	//
	async fn bind<'a>( sock_addr: &'a str ) -> Result< UnixStream, failure::Error >
	{
		let     listener   = UnixListener::bind( sock_addr )?;
		let mut connection = listener.incoming();

		if let Some( income ) = await!( connection.next() )
		{
			// Return has to be here! We want to break from loop and function when we are connected.
			// We only allow one connection atm. It's not great security, but we only want our child
			// process to connect to us, so not allowing further connections.
			//
			// This does mean that if the connection would drop, child process cannot reconnect but needs to be
			// given a new socket, which currently is not implemented. That being said, on unix sockets, this
			// shouldn't be a problem in real life, but this is most certainly temporary code.
			//
			// TODO: Make secure ipc channel
			//
			return Ok( income? )
		};

		Err( EkkeError::NoConnectionsReceived )?
	}
}



impl fmt::Debug for App
{
	fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result
	{
		write!
		(
			f,
			"App {{ name: {}, path: {:?}, routes: {:#?}, services: {:#?} }}",
			self.name, self.path, self.routes, self.services
		)
	}
}


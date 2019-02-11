use failure:: *                                    ;
use crate  :: { IpcHandler                       } ;



pub type EkkeResult<T> = std::result::Result<T, failure::Error>;


#[ derive( Debug, Fail ) ]
//
pub enum EkkeError
{
	#[ fail( display = "No handler registered for service: {}", _0 ) ]
	//
	NoHandlerForService( String ),

	#[ fail( display = "Dispatcher: Handler for service already registered: {}, by actor: {:?}", _0, _1 ) ]
	//
	DoubleServiceRegistration( String, IpcHandler ),

	#[ fail( display = "Bind to unix socket: Nobody connected to the socket" ) ]
	//
	NoConnectionsReceived
}


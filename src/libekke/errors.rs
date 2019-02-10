use failure:: *              ;
use crate  :: { IpcHandler } ;



pub type EkkeResult<T> = std::result::Result<T, failure::Error>;


#[ derive( Debug, Fail ) ]
//
pub enum EkkeError
{
	#[ fail( display = "Handler for service already registered: {}, by actor: {:?}", _0, _1 ) ]
	//
	DoubleServiceRegistration( String, IpcHandler ),

	#[ fail( display = "Nobody connected to the socket" ) ]
	//
	NoConnectionsReceived
}


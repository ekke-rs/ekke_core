use crate :: { import::* };


pub type EkkeResult<T> = std::result::Result<T, failure::Error>;


#[ derive( Debug, Fail ) ]
//
pub enum EkkeError
{
	#[ fail( display = "Bind to unix socket: Nobody connected to the socket" ) ]
	//
	NoConnectionsReceived,

	#[ fail( display = "Tried to register route twice, route: {}", _0 ) ]
	//
	DoubleRouteRegistration( String ),
}


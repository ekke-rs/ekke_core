use failure::*;

pub type EkkeResult<T> = std::result::Result<T, failure::Error>;


#[ derive( Debug, Fail ) ]
//
pub enum EkkeError
{
	#[ fail( display = "Cannot use socket before connecting" ) ]
	//
	UseSocketBeforeConnect,

	#[ fail( display = "Nobody connected to the socket" ) ]
	//
	NoConnectionsReceived
}


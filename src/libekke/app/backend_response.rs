use
{
	actix    :: { prelude::*             },
	serde    :: { Serialize, Deserialize },
	typename :: { *                      },

   hyper    :: { StatusCode             },
};

/// Response sent by an application to it's frontend
///
#[ derive( Debug, Clone, Serialize, Deserialize, Message, TypeName ) ]
//
pub struct BackendResponse
{
	pub status: Status   ,
	pub body  : Vec<u8>  ,
}


/// Pseudo http status codes for [BackendResponse], to indicate success or failure
///
#[ derive( Debug, Clone, Serialize, Deserialize, Message, TypeName ) ]
//
pub enum Status
{
	Ok                  ,
	NotFound            ,
	Fobidden            ,
	InternalServerError ,
	PermanentlyMoved    ,
}


impl From< Status > for hyper::StatusCode
{
	fn from( status: Status ) -> Self
	{
		match status
		{
			Status::Ok                  => StatusCode::OK                    ,
			Status::NotFound            => StatusCode::NOT_FOUND             ,
			Status::Fobidden            => StatusCode::FORBIDDEN             ,
			Status::PermanentlyMoved    => StatusCode::MOVED_PERMANENTLY     ,
			Status::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR ,
		}
	}
}

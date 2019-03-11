use crate :: { import::* };



/// The type apps will receive when frontends make requests.
///
#[ derive( Debug, Clone, Serialize, Deserialize, Message, TypeName ) ] #[ rtype( result="IpcMessage" ) ]
//
pub struct FrontendRequest
{
	pub conn_id : ConnID ,
	pub path    : String ,
	pub payload : Vec<u8>,
}

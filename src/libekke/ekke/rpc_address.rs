use
{
	actix    :: { prelude::*             },
	serde    :: { Serialize, Deserialize },
	typename :: { *                      },

   ekke_io  :: { Rpc                    },
   crate    :: { Ekke                   },
};


#[ derive( Debug, Clone, Serialize, Deserialize, Message, TypeName ) ] #[ rtype( result="Addr<Rpc>" ) ]
//
pub struct RpcAddress;



impl Handler<RpcAddress> for Ekke
{
	type Result = Addr<Rpc>;

	fn handle( &mut self, _msg: RpcAddress, _ctx: &mut Context<Self> ) -> Self::Result
	{
		self.rpc.clone()
	}
}


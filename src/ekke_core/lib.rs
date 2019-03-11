//! This is the actual functionality for the ekke framework server. The binary contains just a very basic main function. All functionality is exposed through this library so you could build against it if needed.
//
#![ feature( await_macro, async_await, futures_api, nll, stmt_expr_attributes, never_type ) ]


pub type PinBoxFut<T> = std::pin::Pin<Box< dyn futures::future::Future< Output = T >>>;

mod app;
mod ekke;
mod ekke_server;
mod errors;
mod config;


pub use app::
{
	App,
	FrontendRequest,
	BackendResponse,
	Status as ResponseStatus,
};


pub use ekke::
{
	Ekke,
	RpcAddress,
};


pub use ekke_server::
{
	EkkeServer
};


use config::
{
	Settings  ,
	AppConfig ,
};


pub use errors::
{
	  EkkeError
	, EkkeResult
};


pub mod services
{
	pub use crate::ekke::RegisterApplication;
	pub use crate::ekke::RegisterApplicationResponse;
}




use
{
	crate   :: { import::*, services::* }
};


pub(crate) fn service_map( rpc: &Rpc, log: Logger, msg: IpcMessage, ipc_peer: Recipient< IpcMessage > )
{
    match msg.service.as_ref()
    {
        "RegisterApplication" => rpc.deser_into::<RegisterApplication>( msg, ipc_peer ),
        _                     => error!( &log, "MainUi: Received request for unknown service: {}", &msg.service ),
    }
}


mod import
{
	#[ allow( unused_imports ) ]
	//
	pub( crate ) use
	{
		ekke_io               :: { ConnID, HttpServer, IpcMessage, IpcPeer, IpcRequestOut,
			                        MessageType, ResponseFuture, Rpc, ThreadLocalDrain                           },
		ekke_config           :: { Config                                                                       },

		actix                 :: { Actor, Addr, Arbiter, AsyncContext, Context,
			                        Handler, Message, Recipient, Supervised, SystemService                       },
		clap                  :: { App as AppCli, Arg, ArgMatches, crate_version, crate_authors                 },
		failure               :: { Fail, Error, ResultExt as _                                                  },

		futures               :: { future::{ join_all, ok }                                                     },
		futures_util          :: { future::FutureExt, try_future::TryFutureExt                                  },

		hashbrown             :: { HashMap                                                                      },
		hyper                 :: { Response, Request, Body, StatusCode                                          },
		lazy_static           :: { lazy_static                                                                  },
		parking_lot           :: { Mutex, RwLock                                                                },
		serde                 :: { Serialize, Deserialize                                                       },

		slog_term             :: { TermDecorator, CompactFormat                                                 },
		slog_async            :: { Async                                                                        },
		slog                  :: { Drain, Logger, trace, debug, info, warn, error, crit, o                      },
		slog_unwraps          :: { ResultExt                                                                    },

		std                   :: { cell::RefCell, convert::From, convert::TryFrom, env, fmt, net::SocketAddr,
			                        path::PathBuf, process::Command, rc::Rc, sync::Arc                           },

		tokio                 :: { net::UnixStream, net::UnixListener                                           },
		tokio_async_await     :: { await as awaits, stream::StreamExt                                           },

		typename              :: { TypeName                                                                     },
	};
}

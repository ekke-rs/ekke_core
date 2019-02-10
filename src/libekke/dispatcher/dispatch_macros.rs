
/// This reduces alot of boilerplate in dispatcher.rs... so here is what it does:
/// 1. Get the service handler from the service name that we get in IpcMessage.
///    If it was not found, call error handling.
/// 2. Try to deserialize the paylod of IpcMessage to a type of the same name of the service
/// 3. Send the deserialized object to the handler
/// 4. Wait for their response and send it back to IpcPeer
///
#[ macro_export ]
//
macro_rules! request_response
{
( $self:ident, $msg:ident, $name:ident ) =>

(
	#[ allow( irrefutable_let_patterns ) ]

	// Get the service handler out of our hashmap
	//
	match $self.handlers.get( "$name" )
	{
		// If we have a handler for this service
		//
		Some( service ) =>
		{

			// Extract the addr of the Actor that handles this service
			//
			if let IpcHandler::$name( addr ) = service
			{
				// Deserialize the payload
				//
				let de: $name = match des( &$msg.ipc_msg.payload )
				{
					Ok ( data  ) => data,
					Err( error ) =>
					{
						Self::error_response
						(
							  format!( "Ekke Server could not deserialize your cbor data for service:{} :{:?}", &$msg.ipc_msg.service, error )
							, $msg.ipc_peer
						);

						// If we can't deserialize the message, there's no point in continuing to handle this request.
						//
						return;
					}
				};


				let addr = addr.clone();

				Arbiter::spawn( async move
				{
					// Get the response from the service actor.
					//
					let resp = await!( addr.send( de ) ).expect( "MailboxError" );

					// Send the response to the peer application through the Ipc channel.
					//
					await!( $msg.ipc_peer.send( resp ) ).expect( "MailboxError" );

					Ok(())

				}.boxed().compat() )
			}
		},

		None => panic!( "No handler registered for service: {}", "$name" )
	}

)
}

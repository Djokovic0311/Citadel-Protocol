use citadel_sdk::prelude::*;
use citadel_sdk::prefabs::server::client_connect_listener::ClientConnectListenerKernel;
use std::net::SocketAddr;
use std::str::FromStr;
use futures::StreamExt;

// ClientConnectListenerKernel implementation provided in the question

#[tokio::main]
async fn main() {
    citadel_logging::setup_log();
    let server_kernel = ClientConnectListenerKernel::new(|connect_success, remote| async move {
        let (sink, mut stream) = connect_success.channel.split();
        
        while let Some(message) = stream.next().await {
            let bytes = b"ping";
            
            // convert message to string
            // println!("{}", msg_str);
            if message == bytes {
                println!("Received 'ping' from client!");
                sink.send_message("pong".to_string().into()).await?;
            }            
        }
        Ok(())
    });
    // this server will listen on 127.0.0.1:25021, and will use the built-in defaults. When calling 'build', a NetKernel is specified
    let server = NodeBuilder::default()
    .with_node_type(NodeType::Server(
        SocketAddr::from_str("127.0.0.1:25021").unwrap(),
    ))
    .build(server_kernel);
    
    // await the server to execute
    let result = server.unwrap().await.unwrap();
}


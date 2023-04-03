use citadel_sdk::prelude::*;
use citadel_sdk::prefabs::client::single_connection::SingleClientServerConnectionKernel;
use futures::{SinkExt, StreamExt};

#[tokio::main]
async fn main() {
    let client_kernel = SingleClientServerConnectionKernel::new_register_defaults("John Doe", "john.doe", "password", "127.0.0.1:25021", |connect_success, mut remote| async move {        
        let (sink, mut stream) = connect_success.channel.split();
        sink.send_message("ping".to_string().into()).await?;
        
        while let Some(message) = stream.next().await {
            let expected_msg = b"pong";
            if message == expected_msg {
                println!("Received 'pong' from server!");
                
                remote.deregister().await?;
                println!("Client deregister");
                break;
            }            
        }       

        Ok(())
    }).unwrap();

    let client = NodeBuilder::default().build(client_kernel);


    match client.unwrap().await {
        Ok(_) => println!("Client finished successfully."),
        Err(e) => eprintln!("Client error: {}", e),
    }


}

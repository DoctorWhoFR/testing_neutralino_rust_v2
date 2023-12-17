use std::process;
use std::sync::Arc;
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio::sync::{Mutex, RwLock};
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;
use typemap_rev::TypeMap;
use crate::message_kind::BasicMessage;

pub async fn message_handler(mut read: SplitStream<WebSocketStream<TcpStream>>, state: Arc<RwLock<TypeMap>>, writer: Arc<Mutex<SplitSink<WebSocketStream<TcpStream>, Message>>>) {
    let writer = writer.clone();
    tokio::spawn(async move {
        loop {
            let msg = read.next().await;
            let msg = match msg {
                None => {
                    println!("no message breaking");
                    process::exit(0);
                }
                Some(msg) => {
                    match msg {
                        Ok(msg) => {
                            match msg {
                                Message::Text(msg) => msg,
                                _ => {
                                    println!("unaccepted message format continue");
                                    continue
                                }
                            }
                        },
                        Err(_) => {
                            println!("invalid message continue");
                            continue
                        }
                    }
                }
            };

            let bmd: BasicMessage = serde_json::from_str(&*msg).unwrap();
            let bmd_clone: BasicMessage = bmd.clone();
            println!("{:?}", bmd);

            let answer = bmd.kind.expect("").execute(bmd_clone, state.clone(), writer.clone()).await;

            match answer {
                None => {}
                Some(answer) => {
                    let mut l_writer = writer.lock().await;

                    let response = serde_json::to_string(&answer);
                    let response = match response {
                        Ok(res) => res,
                        Err(err) => {
                            println!("{:?}", err);
                            continue
                        }
                    };

                    let d = l_writer.send(Message::Text(response)).await;
                    println!("{:?}", d);
                }
            }
        };

    });
}

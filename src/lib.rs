use std::sync::Arc;
use std::time::Duration;
use colored::Colorize;
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio::sync::{Mutex, RwLock};
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;
use crate::basic_message::BasicMessage;
use crate::electron::init_electron;
use crate::tcp_listener::retrieve_connection;

mod basic_message;
mod electron;
mod init_message;
mod message_handler;
mod message_kind;
mod tcp_listener;

async fn message_handler(mut read: SplitStream<WebSocketStream<TcpStream>>, state: Arc<RwLock<GlobalState>>, writer: Arc<Mutex<SplitSink<WebSocketStream<TcpStream>, Message>>>) {
    let writer = writer.clone();
    tokio::spawn(async move {
        loop {
            let msg = read.next().await;
            let msg = match msg {
                None => {
                    println!("no message breaking");
                    break
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
            let answer = bmd.kind.test(state.clone(), writer.clone());

            match answer {
                None => {}
                Some(answer) => {
                    let mut l_writer = writer.lock().await;
                    let _ = l_writer.send(Message::Text("fezfz".to_string())).await;

                }
            }
        };
    });
}

struct GlobalState {
    stop_state: bool
}

#[tokio::test]
async fn test(){
    let mut connection_counter = 0;
    let core = init_electron(vec!["InitMessage", "PalaBar"], vec!["ConfigUpdate"], vec!["StartBot", "StopBot", "Capture"]);

    let state = GlobalState {
        stop_state: false
    };

    let arc_state = Arc::new(RwLock::new(state));

    loop {
        let (writer, mut read) = retrieve_connection().await;
        let arc_writer = Arc::new(Mutex::new(writer));

        println!("connection established {}", connection_counter);
        connection_counter = connection_counter + 1;

        message_handler(read, arc_state.clone(), arc_writer.clone()).await;

        loop {
            if arc_state.read().await.stop_state {
                break;
            }

            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }
}
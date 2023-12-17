use std::sync::{Arc};
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

// pub async fn message_handler(boxed_writer: Arc<Mutex<SplitSink<WebSocketStream<TcpStream>, Message>>>, mut read: SplitStream<WebSocketStream<TcpStream>>, config: Arc<Mutex<HandlerState>>) {
//     let connection_counter = 0;
//     println!("event handler spawned");
//     loop {
//         let t = read.next().await;
//         let t = match t {
//             None => {
//                 println!("connection closed try reconnect after {}", connection_counter);
//                 break;
//             }
//             Some(msg_res) => {
//                 match msg_res {
//                     Ok(msg) => {
//                         match msg {
//                             Message::Text(msg) => msg,
//                             _ => {
//                                 println!("unaccepted time received {} {:?}", connection_counter, msg);
//                                 continue
//                             }
//                         }
//                     },
//                     Err(_) => {
//                         println!("connection closed try reconnect after {}", connection_counter);
//                         break;
//                     }
//                 }
//             }
//         };
//
//         let basic_message: serde_json::error::Result<BasicMessage> = serde_json::from_str(&*t);
//
//         match basic_message {
//             Ok(bm) => {
//                 let handler_writer_clone = boxed_writer.clone();
//                 let answer = bm.execute(handler_writer_clone, config.clone()).await;
//                 match answer {
//                     None => {}
//                     Some(answer) => {
//                         let answer_res = answer.send(boxed_writer.clone()).await;
//                         println!("Answer result : {:?}", answer_res);
//                     }
//                 }
//             }
//             Err(err) => {
//                 println!("not a valid basic msg {}", err)
//             }
//         };
//     };
// }
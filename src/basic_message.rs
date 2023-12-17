use serde::{Deserialize, Serialize};
use crate::message_kind::{MessageKindTrait};


#[derive(Serialize, Deserialize, Debug)]
pub struct BasicMessage {
    pub id: String,
    pub kind: Box<dyn MessageKindTrait>
}

// impl BasicMessage {
//     pub(crate) async fn execute(&self, handler_writer_clone: Arc<Mutex<SplitSink<WebSocketStream<TcpStream>, Message>>>, config: Arc<Mutex<HandlerState>>) -> Option<BasicMessage> {
//         match &self.kind {
//             MessageKind::PinPong(sub) => {
//                 sub.handle(self.clone(), handler_writer_clone).await
//             },
//             (not_handled_kind) => {
//                 println!("not handled {:?}", not_handled_kind);
//                 None
//             }
//         }
//     }
//
//     pub(crate) async fn send(&self, handler_writer_clone: Arc<Mutex<SplitSink<WebSocketStream<TcpStream>, Message>>>) -> Result<(), String> {
//         let message = serde_json::to_string(&self);
//
//         let message = match message {
//             Ok(string) => string,
//             Err(err) => {
//                 println!("cant answer {}", err);
//                 return Err("Can't deserializes".to_string())
//             }
//         };
//
//         let mut lock = handler_writer_clone.lock().await;
//         lock.send(Message::Text(message)).await.expect("TODO: panic message");
//
//         Ok(())
//     }
// }

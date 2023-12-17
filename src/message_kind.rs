use serde::{Deserialize, Serialize};
use core::fmt::Debug;
use std::sync::Arc;
use futures_util::stream::SplitSink;
use tokio::net::TcpStream;
use tokio::sync::{Mutex, RwLock};
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;
use websocket::header::Basic;
use crate::basic_message::BasicMessage;
use crate::GlobalState;

#[typetag::serde(tag = "type")]
pub(crate) trait MessageKindTrait {
    fn test(&self, state: Arc<RwLock<GlobalState>>, writer: Arc<Mutex<SplitSink<WebSocketStream<TcpStream>, Message>>>) -> Option<BasicMessage>;
}

impl Debug for dyn MessageKindTrait {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Series")
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pong(pub Option<String>);

#[typetag::serde]
impl MessageKindTrait for Pong {
    fn test(&self, state: Arc<RwLock<GlobalState>>, writer: Arc<Mutex<SplitSink<WebSocketStream<TcpStream>, Message>>>) -> Option<BasicMessage> {
        println!("test {:?}", &self);

        None
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MessageKind {
    Capture(Option<String>)
}

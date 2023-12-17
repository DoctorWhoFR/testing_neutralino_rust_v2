use serde::{Deserialize, Serialize};
use core::fmt::Debug;
use std::sync::Arc;
use async_trait::async_trait;
use futures_util::stream::SplitSink;
use tokio::net::TcpStream;
use tokio::sync::{Mutex, RwLock};
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;
use typemap_rev::TypeMap;
use crate::GlobalState;

#[async_trait]
#[typetag::serde(tag = "type")]
pub(crate) trait MessageExecute: Send {
    async fn execute(&self, msg: BasicMessage, state: Arc<RwLock<TypeMap>>, writer: Arc<Mutex<SplitSink<WebSocketStream<TcpStream>, Message>>>) -> Option<BasicMessage>;
}

impl Debug for dyn MessageExecute {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Series")
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BasicMessage {
    pub id: String,
    pub kind: Option<Box<dyn MessageExecute>>
}

impl Clone for BasicMessage {
    fn clone(&self) -> Self {
        return Self {
            id: self.id.clone(),
            kind: None,
        }
    }

    fn clone_from(&mut self, source: &Self) {
        todo!()
    }
}
use std::sync::Arc;
use async_trait::async_trait;
use futures_util::stream::SplitSink;
use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;
use tokio::sync::{Mutex, RwLock};
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;
use typemap_rev::TypeMap;
use crate::GlobalState;
use crate::message_kind::{BasicMessage, MessageExecute};

#[derive(Serialize, Deserialize, Debug)]
pub struct PingPong(pub Option<String>);

#[async_trait]
#[typetag::serde]
impl MessageExecute for PingPong {
    async fn execute(&self, msg: BasicMessage, state: Arc<RwLock<TypeMap>>, writer: Arc<Mutex<SplitSink<WebSocketStream<TcpStream>, Message>>>) -> Option<BasicMessage> {
        println!("test {:?}", &self);

        Some(BasicMessage{
            id: msg.id.clone(),
            kind: Some(Box::new(PingPong(Some("pong".to_string()))))
        })
    }
}

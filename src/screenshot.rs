use std::sync::Arc;
use async_trait::async_trait;
use futures_util::stream::SplitSink;
use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;
use tokio::sync::{Mutex, RwLock};
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;
use typemap_rev::TypeMap;
use crate::message_kind::{BasicMessage, MessageExecute};
use screenshots::Screen;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Screenshot(Option<String>);

#[async_trait]
#[typetag::serde]
impl MessageExecute for Screenshot {
    async fn execute(&self, msg: BasicMessage, state: Arc<RwLock<TypeMap>>, writer: Arc<Mutex<SplitSink<WebSocketStream<TcpStream>, Message>>>) -> Option<BasicMessage> {
        let screen = Screen::from_point(0, 0).unwrap();
        let image = screen.capture().unwrap();

        let uuid = Uuid::new_v4();
        image.save(format!("{}.png", uuid.to_string())).expect("TODO: panic message");

        Some(BasicMessage{
            id: msg.id.clone(),
            kind: Some(Box::new(Screenshot(Some(uuid.to_string()))))
        })
    }
}

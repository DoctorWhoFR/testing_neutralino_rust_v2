use std::sync::Arc;
use async_trait::async_trait;
use futures_util::stream::SplitSink;
use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;
use tokio::sync::{Mutex, RwLock};
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;
use typemap_rev::TypeMap;
use websocket::futures::Future;
use crate::GlobalState;
use crate::message_kind::{BasicMessage, MessageExecute};

#[derive(Serialize, Deserialize, Debug)]
pub struct StopSignal;

#[async_trait]
#[typetag::serde]
impl MessageExecute for StopSignal {
    async fn execute(&self, msg:BasicMessage, state: Arc<RwLock<TypeMap>>, writer: Arc<Mutex<SplitSink<WebSocketStream<TcpStream>, Message>>>) -> Option<BasicMessage> {
        let mut state_lock = state.write().await;
        let mut state = state_lock.get_mut::<GlobalState>().unwrap();
        state.stop_state = true;

        None
    }
}

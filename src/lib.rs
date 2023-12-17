use std::sync::Arc;
use std::time::Duration;
use colored::Colorize;
use futures_util::{SinkExt, StreamExt};
use serde::Serialize;
use tokio::sync::{Mutex, RwLock};
use typemap_rev::{TypeMap, TypeMapKey};
use crate::electron::init_electron;
use crate::message_handler::message_handler;
use crate::tcp_listener::retrieve_connection;

mod electron;
mod init_message;
mod message_handler;
mod message_kind;
mod tcp_listener;
mod pingpong;
mod stopsignal;
mod screenshot;

#[derive(Debug, Clone, Copy)]
struct GlobalState {
    stop_state: bool
}

impl TypeMapKey for GlobalState {
    type Value = GlobalState;
}

#[tokio::test]
async fn test(){
    let mut connection_counter = 0;
    let core = init_electron(
        vec![],
        vec!["PingPong", "Screenshot"],
        vec!["StopSignal"]
    );

    'main: loop {
        let (writer, mut read) = retrieve_connection().await;
        let arc_writer = Arc::new(Mutex::new(writer));

        println!("connection established {}", connection_counter);
        connection_counter = connection_counter + 1;

        let arc_state = Arc::new(RwLock::new(TypeMap::new()));
        arc_state.write().await.insert::<GlobalState>(GlobalState {
            stop_state: false
        });

        message_handler(read, arc_state.clone(), arc_writer.clone()).await;

        loop {
            let global_lock = arc_state.read().await;
            let state = global_lock.get::<GlobalState>().unwrap();
            println!("{:?}", state);
            if state.stop_state {
                println!("received stop signal break");
                break 'main;
            }
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }
    println!("connection loop ended trow stop signal")

}
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::StreamExt;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;

async fn accept_connection(stream: TcpStream, connection_counter: i32) -> (SplitSink<WebSocketStream<TcpStream>, Message>, SplitStream<WebSocketStream<TcpStream>>) {
    let addr = stream.peer_addr().expect("connected streams should have a peer address");
    println!("Peer address: {} {}", connection_counter, addr);

    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");

    println!("New WebSocket connection: {} {}", connection_counter,  addr);

    let (write, mut read) = ws_stream.split();

    (write, read)
}

pub async fn retrieve_connection() -> (SplitSink<WebSocketStream<TcpStream>, Message>, SplitStream<WebSocketStream<TcpStream>>) {
    let try_socket = TcpListener::bind("127.0.0.1:8080").await;
    let listener = try_socket.expect("Failed to bind");

    let listener = listener.accept().await.unwrap();
    accept_connection(listener.0, 0).await
}

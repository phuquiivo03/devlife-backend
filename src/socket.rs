use socketioxide::extract::SocketRef;
use tracing::info;

pub async fn on_connect(socket: SocketRef) {
    info!("socket connected {}", socket.id);
}

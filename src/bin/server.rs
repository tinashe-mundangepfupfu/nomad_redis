use tokio::net::TcpListener;
use bytes::BytesMut;
use tokio::io::AsyncReadExt;
use nomad_redis::Command;
use nomad_redis::helper::buffer_to_array;

// transform async main to a sync function
#[tokio::main]
pub async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:8001").await?;
    loop {
        let(mut socket, _) = listener.accept().await?;  // Wait for connection, listening on single thread.
        println!("connection accepted {:?}", socket);

        let mut buf = BytesMut::with_capacity(1024);
        socket.read_buf(&mut buf).await?;
        println!("buffer {:?}", buf); // printing the data in the buffer

        let attrs = buffer_to_array(&mut buf);
        let command = Command::get_command(&attrs[0]);
    }
    Ok(())
}
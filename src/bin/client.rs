use bytes::BytesMut;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
pub async fn main() -> Result<(), std::io::Error> {
    let mut stream = TcpStream::connect("127.0.0.1:8001").await?;
    stream.write_all(b"set foo bar").await?;

    let mut buf = BytesMut::with_capacity(1024);
    let _length = stream.read_buf(&mut buf).await?;
    match std::str::from_utf8(&mut buf) {
        Ok(resp) => {
            if resp == "r Ok" {
                println!("key updated");
            } else if resp == "Ok" {
                println!("key set");
            }
        }
        Err(err) => {
            // failed to convert bytes into string slice
            println!("error: {}", err);
        }
    }
    Ok(())
}
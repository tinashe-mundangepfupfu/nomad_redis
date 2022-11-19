use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

#[tokio::main]
pub async fn main() -> Result<(), std::io::Error> {
    let mut stream = TcpStream::connect("127.0.0.1:8001").await?;
    stream.write_all(b"Set foo bar").await?;
    Ok(())
}
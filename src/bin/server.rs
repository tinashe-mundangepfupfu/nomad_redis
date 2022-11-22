use tokio::net::{TcpListener, TcpStream};
use bytes::BytesMut;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use nomad_redis::{Command, Db};
use nomad_redis::helper::buffer_to_array;

// transform async main to a sync function
#[tokio::main]
pub async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:8001").await?;
    let mut db = Db::new();
    loop {
        let(mut socket, _) = listener.accept().await?;  // Wait for connection, listening on single thread.
        println!("connection accepted {:?}", socket);

        let mut buf = BytesMut::with_capacity(1024);
        socket.read_buf(&mut buf).await?;
        println!("buffer {:?}", buf); // printing the data in the buffer

        let attrs = buffer_to_array(&mut buf);
        let command = Command::get_command(&attrs[0]);
        process_query(command, attrs, &mut socket, &mut db).await?;
    }

}

async fn process_query(command: Command, attrs: Vec<String>, socket: &mut TcpStream, db: &mut Db,
) -> std::io::Result<()> {
    match command {
        Command::Get => {
            Ok(())
        }
        Command::Set => {
            let resp = db.write(&attrs);
            match resp {
                Ok(result) => {
                    println!("Set result: {}", result);
                }
                Err(_err) => {
                    socket.write_all(b"").await?;
                }
            }
            Ok(())
        }
        Command::Invalid => Ok(()),
    }
}
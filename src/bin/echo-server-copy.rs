use tokio::{io,net::TcpListener};

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6142").await?;

    loop {
        let (mut socket, _addr) = listener.accept().await?;

        tokio::spawn(async move {
            let (mut r, mut w) = socket.split();
            if io::copy(&mut r, &mut w).await.is_err() {
                eprintln!("failed to copy");
            }
        });
    }
}

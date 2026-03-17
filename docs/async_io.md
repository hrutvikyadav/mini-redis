# TOC
- # I/O
- ## AsyncRead and AsyncWrite
- ### async fn read()
- ### async fn read_to_end()
- ### async fn write()
- ### async fn write_all()
- ### Helper functions

# I/O
I/O in Tokio operates in much the same way as in std, but asynchronously.

There is a *trait for reading (AsyncRead) and a trait for writing (AsyncWrite)*.
> [!hint]
> Specific *types implement these traits* as appropriate (TcpStream, File, Stdout). AsyncRead and AsyncWrite are *also implemented by a number of data structures*, such as Vec<u8> and &[u8].
> This allows using byte arrays where a reader or writer is expected.

This page will cover **basic I/O reading and writing with Tokio** and work through a few examples.

## AsyncRead and AsyncWrite
These two traits provide the facilities to asynchronously read from and write to byte streams.
The *methods on these traits are typically not called directly*, similar to how you don't manually call the poll method from the Future trait. Instead, you will *use them through the utility methods* provided by AsyncReadExt and AsyncWriteExt.

Let's briefly look at a few of these methods.
All of these functions are async and must be used with .await.

### async fn read()
AsyncReadExt::read provides an async method for reading data into a buffer, returning the number of bytes read.


Note: when read() returns Ok(0), this signifies that the stream is closed. Any further calls to read() will complete immediately with Ok(0). With TcpStream instances, this signifies that the read half of the socket is closed.
```rs
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut f = File::open("foo.txt").await?;
    let mut buffer = [0; 10];

    // read up to 10 bytes
    let n = f.read(&mut buffer[..]).await?;

    println!("The bytes: {:?}", &buffer[..n]);
    Ok(())
}
```

### async fn read_to_end()
AsyncReadExt::read_to_end reads all bytes from the stream until EOF.
```rs
use tokio::io::{self, AsyncReadExt};

use tokio::fs::File;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut f = File::open("foo.txt").await?;
    let mut buffer = Vec::new();

    // read the whole file
    f.read_to_end(&mut buffer).await?;
    Ok(())
}
```

### async fn write()
AsyncWriteExt::write writes a buffer into the writer, returning how many bytes were written.
```rs
use tokio::io::{self, AsyncWriteExt};
use tokio::fs::File;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut file = File::create("foo.txt").await?;

    // Writes some prefix of the byte string, but not necessarily all of it.
    let n = file.write(b"some bytes").await?;

    println!("Wrote the first {} bytes of 'some bytes'.", n);
    Ok(())
}
```

### async fn write_all()
AsyncWriteExt::write_all writes the entire buffer into the writer.
```rs
use tokio::io::{self, AsyncWriteExt};
use tokio::fs::File;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut file = File::create("foo.txt").await?;

    file.write_all(b"some bytes").await?;
    Ok(())
}
```

Both traits include a number of other helpful methods. See the API docs for a comprehensive list.

### Helper functions
Additionally, just like std, the tokio::io module contains a number of helpful utility functions as well as APIs for working with standard input, standard output and standard error.

For example, tokio::io::copy asynchronously copies the entire contents of a reader into a writer.
```rs
use tokio::fs::File;
use tokio::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut reader: &[u8] = b"hello";
    let mut file = File::create("foo.txt").await?;

    io::copy(&mut reader, &mut file).await?;
    Ok(())
}
```

> [!hint]
> Note that this uses the fact that byte arrays also implement AsyncRead.

## Practice with echo server

Create a TCP echo server
- binds to port and listens -> accepts connection*s*
- reads from socket
- writes back what was read (echo)

1 Implementation 2 strategies
### Using io::copy()
socket implements both reader and writer but `io::copy(&mut socket, &mut socket).await` does not work (mut)
> solution
> split reader + writer

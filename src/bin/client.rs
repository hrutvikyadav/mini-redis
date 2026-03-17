use mini_redis::client;
use tokio::sync::oneshot;
use bytes::Bytes;

#[derive(Debug)]
enum CommandType {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        value: Bytes,
        resp: Responder<()>,
    }
}

type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = tokio::sync::mpsc::channel(32);
    let tx2 = tx.clone();

    let manager = tokio::spawn(async move {
        // Establish a connection to the server
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();

        while let Some(cmd) = rx.recv().await {
            match cmd {
                CommandType::Get { key, resp } => {
                    let res = client.get(&key).await;
                    let _ = resp.send(res);
                }
                CommandType::Set { key, value, resp } => {
                    let res = client.set(&key, value).await;
                    let _ = resp.send(res);
                }
            }
        }
    });

    // Spawn two tasks, one gets a key, the other sets a key
    let t1 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        // let res = client.get("foo").await; WARN: not possible
        let cmd =  CommandType::Get { key: "foo".to_string(), resp: resp_tx };
        tx.send(cmd).await.unwrap();

        let res = resp_rx.await;
        println!("Got {:?}", res);
    });

    let t2 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        // client.set("foo", "bar".into()).await; WARN: not possible
        let cmd =  CommandType::Set { key: "foo".to_string(), value: "bar".into(), resp: resp_tx };
        tx2.send(cmd).await.unwrap();

        let res = resp_rx.await;
        println!("Got {:?}", res);
    });

    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();
}

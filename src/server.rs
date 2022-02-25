use std::collections::HashMap;
use std::future::Future;

use tokio::net::TcpListener;
use tokio::sync::mpsc;

use crate::connection::Connection;

pub async fn run(listener: TcpListener, shutdown: impl Future) {
    let mut server = Server{ 
        listener,
    };
    tokio::select! {
        res = server.run() => {
            if let Err(err) = res {
                println!("failed to accept, error: {}", err);
            }
        }
        _ = shutdown => {
            println!("shutting down");
        }
    }
    // TODO: shutdown gracefully
}

struct Server {
    listener: TcpListener,
}

impl Server {
    async fn run(&mut self) -> crate::Result<()> {
        println!("accepting requests ...");
        let (tx, mut rx) = 
            mpsc::channel(16);

        tokio::spawn(async move {
            let mut db = HashMap::new();
            while let Some(req) = rx.recv().await {
                crate::db::handle_request(req, &mut db).await;
            }
        });

        loop {
            let (socket, _addr) = self.listener.accept().await.unwrap();
            println!("accepted a request");
            let connection = Connection::new(socket);
            let new_tx = tx.clone();

            tokio::spawn(async move {
                crate::reader::tcp_reader(new_tx, connection).await
            });
        }
    }
}
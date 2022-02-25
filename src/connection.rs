use tokio::{net::{TcpStream, tcp::{OwnedReadHalf, OwnedWriteHalf}}, io::{AsyncWriteExt, BufReader, AsyncBufReadExt}};
use crate::apis::{RequestMessage, ResponseMessage};

pub struct Connection {
    // stream: TcpStream,
    read_stream: BufReader<OwnedReadHalf>,
    write_stream: OwnedWriteHalf,
}

impl Connection {
    pub fn new(socket: TcpStream) -> Connection {
        let (reader, writer) = socket.into_split();
        let buf_reader = BufReader::new(reader);
        Connection { 
            // stream: socket 
            read_stream: buf_reader, 
            write_stream: writer, 
        }
    }

    async fn write_str(&mut self, message: String) -> std::io::Result<()> {
        self.write_stream.write_all(message.as_bytes()).await?;
        self.write_stream.write_all(b"\n").await?;
        self.write_stream.flush().await
    }

    pub(crate) async fn write_request(&mut self, req: RequestMessage) -> std::io::Result<()> {
        let message = serde_json::to_string(&req).unwrap();
        self.write_str(message).await
    }

    pub(crate) async fn read_response(&mut self) -> std::io::Result<ResponseMessage> {
        let mut buf = String::new();
        self.read_stream.read_line(&mut buf).await?;
        match serde_json::from_str(&buf) {
            Ok(resp) => Ok(resp), 
            Err(_e) => Err(std::io::Error::new(std::io::ErrorKind::Other, format!("read_response from: {}", buf))),
        }
    }

    pub(crate) async fn write_response(&mut self, resp: ResponseMessage) -> std::io::Result<()> {
        let message = serde_json::to_string(&resp).unwrap();
        self.write_str(message).await 
    }

    pub(crate) async fn read_request(&mut self) -> std::io::Result<RequestMessage> {
        let mut buf = String::new();
        self.read_stream.read_line(&mut buf).await?;
        match serde_json::from_str(&buf) {
            Ok(req) => Ok(req),
            Err(_e) => Err(std::io::Error::new(std::io::ErrorKind::Other, format!("read_request from: {}", buf))),
        }
    }
}

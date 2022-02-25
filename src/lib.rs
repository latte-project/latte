mod db;
pub mod client;
pub mod server;
mod connection;
pub mod func;
pub mod apis;
mod reader;

// pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Error = std::io::Error;
pub type Result<T> = std::result::Result<T, Error>;
pub type ObjectRef = String;
pub mod apis;
pub mod client;
mod connection;
mod db;
pub mod func;
mod reader;
pub mod server;

// pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Error = std::io::Error;
pub type Result<T> = std::result::Result<T, Error>;
pub type ObjectRef = String;

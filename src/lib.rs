#[macro_use]
extern crate serde_derive;
// extern crate futures;
extern crate serde_json;
extern crate serde_qs as qs;

extern crate itertools;

pub use error::*;
use tokio::net::ToSocketAddrs;

pub mod api;
pub mod connection;
mod driver;
mod error;
pub mod model;
mod spielwiese;

mod contoller;
pub use contoller::Controller;
pub use contoller::Volume;

use crate::connection::Connection;
pub use crate::driver::HeosDriver;

// TODO move to a playground
// pub mod reactive;
pub type HeosResult<T> = Result<T, HeosError>;

pub async fn connect<A>(ip: Option<A>) -> HeosResult<Connection>
where
    A: ToSocketAddrs,
{
    match ip {
        Some(a) => connection::Connection::connect(a).await,
        None => connection::Connection::find().await,
    }
}
pub async fn create_driver(connection: Connection) -> HeosResult<HeosDriver> {
    HeosDriver::new(connection).await
}

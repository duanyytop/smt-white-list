#![feature(test)]
#[macro_use]
extern crate diesel;
extern crate dotenv;

use crate::models::helper::{init_connection_pool, SqlConnectionPool};
use lazy_static::lazy_static;

pub mod models;
pub mod schema;
pub mod utils;

lazy_static! {
    static ref POOL: SqlConnectionPool = init_connection_pool();
}

fn main() {
    println!("Hello, world!");
}

use crc::{Crc, CRC_32_ISO_HDLC};
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{self, ConnectionManager, Pool};
use std::env;

pub type SqlConnectionPool = Pool<ConnectionManager<MysqlConnection>>;

pub const PAGE_SIZE: i64 = 1000;

pub fn init_connection_pool() -> SqlConnectionPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    r2d2::Pool::builder().max_size(20).build(manager).unwrap()
}

pub fn generate_crc(v: &[u8]) -> u32 {
    const CRC: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);
    CRC.checksum(v)
}

pub fn parse_lock_hash(lock_hash: [u8; 32]) -> (String, u32) {
    (
        hex::encode(lock_hash),
        generate_crc(hex::encode(lock_hash).as_bytes()),
    )
}

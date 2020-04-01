pub mod core;
use crate::core::transaction_log::chunk_db;
fn main() {
    let chunk = chunk_db::ChunkDb::open("D:\\databases\\projections-unhandled-bytes-v1\\chunk-000000.000000"
            .parse()
            .unwrap(),
    );
    println!("Header: {:?}\n", &chunk.header);
    println!("Footer: {:?}\n", &chunk.footer);
}

mod core;

fn main() {
    let chunk = core::Chunk::open("D:\\databases\\first\\chunk-000000.000000".parse().unwrap());
}

mod core;

fn main() {
    let chunk = core::Chunk::open(
        "D:\\databases\\performance\\chunk-000000.000000"
            .parse()
            .unwrap(),
    );
    println!("Header: {:?}\n", &chunk.header);
    println!("Footer: {:?}\n", &chunk.footer);
}

fn main() {
    println!("POSTGRES_PASSWORD=\"{}\"", generate_secret())
}

fn generate_secret() -> String {
    let bytes: [u8; 32] = rand::random();
    hex::encode(bytes)
}

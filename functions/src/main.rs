fn main() {
    println!("Plus {}", plus_one(1));
    println!("Plus {}", plus_one(4));
}

fn plus_one(x: u32) -> u32 {
    x + 1
}

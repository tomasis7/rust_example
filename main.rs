fn main() {
    println!("Hello, world!");  
}

let mut name = String::new();

io::stdin()
    .read_line(&mut name)
    .expect("Failed to read line");

  println!("Hello, {}! Welcome to Rust.", name.trim());
}
fn main() {
    let path = "./src/main.rs";
    match std::fs::read_to_string(path) {
        Ok(content) => print!("{}", content),
        Err(why) => println!("{}", why)        
    }
}

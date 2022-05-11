fn main() {

    let mut args = std::env::args();

    match args.nth(1) {
        Some(path) => run(path),
        None => println!("1つ目の実行時引数にファイルパスを入れる必要があります。")
    }
}

fn run(path: String) {
    match std::fs::read_to_string(path) {
        Ok(content) => print!("{}", content),
        Err(why) => println!("{}", why)
    }
}

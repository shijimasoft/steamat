pub fn abort(msg: &str) {
    println!("{}", msg);
    std::process::exit(0);
}
pub fn run() {
    // print env variable in module
    println!("{}", dotenv::var("BAR").unwrap());
}
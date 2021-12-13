mod module;

fn main() {
    // load environment variables from .env
    dotenv::dotenv().expect("Couldn't read .env file!");

    // print env variable in main
    println!("{}", dotenv::var("FOO").unwrap());

    // trying print variable in submodule
    module::run();
}

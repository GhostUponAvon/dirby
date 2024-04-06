use std::{env, fs};
use dirby::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = Config.new(&args).unwrap();

}

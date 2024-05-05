use crate::generate;
use crate::persist;
use clap::Parser;
use create::types;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {}

fn main() {
    println!("Hello, world!");
}

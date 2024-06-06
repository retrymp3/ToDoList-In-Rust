use std::env;
use std::mem;

fn main() {
    let arglen = (env::args_os().len());
    let mut args: Vec<String> = env::args().collect(); // All the command line arguments will be stored in a vector.

    for i in 1..arglen{
        println!("{}", args[i])
    }
}
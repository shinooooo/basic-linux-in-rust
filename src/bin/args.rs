use std::env;

fn main() {
    let args = env::args();

    for (k, v) in args.enumerate() {
        println!("args[{}] = {}", k, v);
    }
}

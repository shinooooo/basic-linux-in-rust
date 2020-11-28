use std::env;
use std::fs::File;
use std::io::{self, BufReader, Read};
use std::process;
fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("{}: flie name not given", args[0]);
        process::exit(1);
    }

    let f = File::open(&args[1])?;
    let mut f = BufReader::new(f);
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;

    // println!("{}", buf.split('\n').count());
    println!("{}", buf.lines().collect::<Vec<&str>>().len());
    Ok(())
}

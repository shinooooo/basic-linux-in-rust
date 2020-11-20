use std::env::args;
use std::fs;
use std::io::{stdout, BufWriter, Write};
use std::process;

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        eprintln!("{}: flie name not given\n", args[0]);
        process::exit(1);
    }

    for arg in args.iter().skip(1) {
        do_cat(arg);
    }
}

fn do_cat(arg: &String) -> () {
    let out = stdout();
    let mut out = BufWriter::new(out.lock());
    let n = fs::read_to_string(arg);
    if n.is_err() {
        eprintln!("{}: can not open\n", arg);
        process::exit(1);
    } else {
        writeln!(out, "{}", n.unwrap()).unwrap();
    }
}

use std::env::args;
use std::fs::{metadata, read_to_string};
use std::io::{stdout, BufWriter, Write};
use std::process;

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        eprintln!("{}: flie name not given", args[0]);
        process::exit(1);
    }

    for arg in args.iter().skip(1) {
        do_cat(arg);
    }
}

fn do_cat(arg: &String) -> () {
    let out = stdout();
    let mut out = BufWriter::new(out.lock());
    let metadata = metadata(arg).unwrap();
    if metadata.is_dir() {
        eprintln!(" {} is directory", arg);
    } else {
        let n = read_to_string(arg);
        if n.is_err() {
            eprintln!("can not open {}", arg);
            process::exit(1);
        } else {
            let n = n.unwrap();
            if writeln!(out, "{}", n).is_err() {
                eprintln!("can not write to stdout");
            };
        }
    }
}

extern crate getopts;
extern crate protobuf;

mod riemann;
mod options;

use getopts::Options;
use std::env;
use self::options::*;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("{} [options] <service>", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let opts = get_options();

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let event = marshall(matches);
    println!("{:?}", event);

//    let input = if !matches.free.is_empty() {
//        matches.free[0].clone()
//    } else {
//        print_usage(&program, opts);
//        return;
//    };
}

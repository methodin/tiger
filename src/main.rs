extern crate md5;
extern crate getopts;
extern crate yaml_rust;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use getopts::Options;
use std::env; 

pub mod project;
mod change;

/**
 * Uses:
 *
 */
fn do_work(directive: &str, mut args: Vec<String>) {
    match directive {
        "init" => project::create(args.as_slice()),
        _ => {
            let mut project = project::load(&directive);

            let mut rest: Vec<_> = args.drain(1..).collect();
            let qualifier = &args[0];

            let args: Vec<_> = rest.drain(1..).collect();
            let action = &rest[0];

            let updated = match qualifier.as_ref() {
                "change" => change::perform(&mut project, &action, &args),
                _ => panic!("{} is not valid", &qualifier),
            };

            if updated {
                project::save(&project);
            }
        }
    }
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!(
        "Usage: {program} FILE [options]
        {program} init TK-123
        {program} TK-123 change set pre my-text.sql 
        {program} TK-123 change ls
        {program} TK-123 change rm hash
        {program} TK-123 change clear
        ",
        program= program
    );
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    // Defined options available for command
    let mut opts = Options::new();
    // opts.optopt("o", "", "set output file name", "NAME");
    opts.optflag("h", "help", "print this help menu");

    // Match available options with args passed in
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    // Print help if using -h flag
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    if matches.free.is_empty() {
        print_usage(&program, opts);
        return;
    }

    // let output = matches.opt_str("o");
    if let Some((directive, rest)) = matches.free.split_first() {
        do_work(&directive, rest.to_vec());
        return;
    }

    print_usage(&program, opts);
}
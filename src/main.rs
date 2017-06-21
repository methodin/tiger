extern crate md5;
extern crate getopts;
extern crate yaml_rust;
extern crate serde;
extern crate serde_json;
extern crate serde_yaml;
extern crate rand;
#[macro_use]
extern crate serde_derive;
extern crate mysql;
extern crate bincode;
extern crate rusoto_core;
extern crate rusoto_s3;
extern crate rusoto_credential;

pub mod project;
mod execute;
pub mod change;
mod package;
pub mod config;

use getopts::{Options,Matches};
use std::env; 
use project::Project;

/**
 * Execute a command against a project or change
 */
fn execute(directive: &str, mut args: Vec<String>, matches:&Matches) {
    match directive {
        "init" => Project::create(&args[0]),
        "up" => execute::run("up", args.as_slice(), &matches),
        "down" => execute::run("down", args.as_slice(), &matches),
        _ => {
            let mut project = project::load(&directive);

            assert!(args.len() > 0, "You must provide at least one parameter");

            let rest: Vec<_> = args.drain(1..).collect();
            let qualifier = &args[0];

            match qualifier.as_ref() {
                "pre" => change::add(&mut project, "pre", &rest),
                "post" => change::add(&mut project, "post", &rest),
                "rm" => change::rm(&mut project, &rest),
                "ls" => project.ls(),
                "clear" => project.clear(),
                "edit" => change::edit(&mut project, &rest),
                "simulate" => execute::simulate(&project, &rest),
                "package" => package::run(project, &matches),
                _ => panic!("{} is an unknown command", qualifier),
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
    opts.optopt("c", "", "set the config file", "CONFIG");
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("r", "run", "execute the the up/down command");

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
        execute(&directive, rest.to_vec(), &matches);
        return;
    }

    print_usage(&program, opts);
}

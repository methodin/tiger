use project::{self,Timing,Project};
use change::{Change,ChangeType};
use std::fs::File;
use std::io::Read;
use serde_yaml;
use getopts;
use mysql as my;
use config;

/**
 * Echoes out all changes to be made 
 */
pub fn simulate(project: &Project, args: &[String]) {
    if args.len() != 1 {
        panic!("You must provide an up or down parameter");
    }

    let direction = match args[0].as_ref() {
        "up" => "up",
        "down" => "down",
        dir => panic!("invalid direction {}", dir),
    };

    let line = format!("{dash:-<100}", dash="-");
    let mut pres: Vec<&Change> = Vec::new();
    let mut posts: Vec<&Change> = Vec::new();

    // Gather timing lists
    for change in &project.changes {
        match change.timing {
            Timing::Pre => pres.push(change),
            Timing::Post => posts.push(change),
        }
    }

    println!("{:>15} {}\n{:>15} {}",
        "> Pre-deploy changes:",
        pres.len(), 
        "> Post-deploy changes:",
        posts.len());

    // Echo pre scripts
    if pres.len() > 0 {
        println!("\n> PRE SCRIPTS\n{}", line);

        for ref change in pres.iter_mut() {
            let content = change.read_file(&project, direction);
            println!("{}", content); 
        }
        println!("{}", line);
    } 

    // Echo post scripts
    if posts.len() > 0 {
        println!("\n> POST SCRIPTS\n{}", line);

        for ref change in posts.iter_mut() {
            let content = change.read_file(&project, direction);
            println!("{}", content); 
        }
        println!("{}", line);
    } 

    println!("> Deployment complete\n");
}

/**
 * Execute one or more projects
 */
pub fn run(direction: &str, args: &[String], matches:&getopts::Matches) {
    if args.len() < 2 {
        panic!("You must provide a timing and at least one project to run");
    }

    let config = config::load_config("run", &matches);

    let projects: &[String] = &args[1..];
    let timing: Timing = args[0].parse::<Timing>()
            .expect("Invalid timing value");
    
    let commit = matches.opt_present("r");

    if !commit {
        println!("Running in simulation mode");
    }

    println!("Connecting to sql server");
    let pool = my::Pool::new(&config.sql.host).unwrap();

    for project_name in projects {
        let project = project::load(&project_name);

        let mut changes: Vec<&Change> = Vec::new();

        // Gather timing lists
        for change in &project.changes {
            if change.timing == timing {
                changes.push(change);
            }
        }

        // Execute pre deploy scripts
        if changes.len() == 0 {
            println!("No changes to run");
            return;
        }

        for ref change in changes.iter_mut() {
            let content = change.read_file(&project, direction);
            match change.change_type {
                ChangeType::Sql => {
                    println!("Executing the following SQL code:\n{}", content); 
                    if commit {
                        pool.prep_exec(&content, ()).unwrap();
                        println!("Success");
                    }
                },
            }
        }

        println!("Migration complete");
    }
}

use project;
use std::fs::{self,File};
use yaml_rust::YamlLoader;
use std::io::Read;

/**
 * Echoes out all changes to be made 
 */
pub fn simulate(project: &project::Project) {
    let line = format!("{dash:-<100}", dash="-");
    let mut pres: Vec<&project::Change> = Vec::new();
    let mut posts: Vec<&project::Change> = Vec::new();

    // Gather timing lists
    for change in &project.changes {
        match change.timing {
            project::Timing::Pre => pres.push(change),
            project::Timing::Post => posts.push(change),
        }
    }

    println!("{:>15} {}\n{:>15} {}",
        "> Pre-deploy changes:",
        pres.len(), 
        "> Post-deploy changes:",
        posts.len());

    if pres.len() > 0 {
        println!("\n> PRE SCRIPTS\n{}", line);

        for ref change in pres.iter_mut() {
            let content = change.read_file(&project);
            println!("{}\n{}", content, line); 
        }
    } 

    if posts.len() > 0 {
        println!("\n> POST SCRIPTS\n{}", line);

        for ref change in posts.iter_mut() {
            let content = change.read_file(&project);
            println!("{}\n{}", content, line); 
        }
    } 

    println!("> Deployment complete\n");
}

/**
 * Execute one or more projects
 */
pub fn run(args: &[String]) {
    //TODO check args
    //
    // Open file
    let mut file = match File::open(&args[0]) {
        Err(_) => panic!("couldn't read {}", args[0]),
        Ok(file) => file,
    };

    // Read file contents
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Could not read config file");

    let docs = YamlLoader::load_from_str(&contents).unwrap();
    let doc = &docs[0];

    println!("{:?}", doc);
}

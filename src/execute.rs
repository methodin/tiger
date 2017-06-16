use project::{Timing,Project};
use change::Change;

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
        dir => panic!("Invalid direction {}", dir),
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

    if pres.len() > 0 {
        println!("\n> PRE SCRIPTS\n{}", line);

        for ref change in pres.iter_mut() {
            let content = change.read_file(&project, direction);
            println!("{}", content); 
        }
        println!("{}", line);
    } 

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
pub fn run(args: &[String]) {
    //TODO check args
    //
    // Open file
    /*
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
    */
}

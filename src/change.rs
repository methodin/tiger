use project;
use md5;
use std::path::Path;
use std::fs::{self,File};
use std::io::prelude::*;

/**
 * Perform a specific change request
 */
pub fn perform(project: &mut project::Project, args: &mut Vec<String>) -> bool {
    match args[0].as_ref() {
        "set" => set(project, &args),
        "ls" => ls(project),
        "clear" => clear(project),
        "rm" => rm(project, &args),
        "sync" => sync(project),
        _ => panic!("Not a valid data action"),
    }
}

/**
 * Create a new change in the current project
 */
fn set(project: &mut project::Project, args: &[String]) -> bool {
    if args.len() < 3 {
        panic!("You must provide at least 2 arguments to change");
    }

    let timing = args[1].to_owned();
    let file = args[2].to_owned();

    // Copy file from source into project
    let project_dir = project.get_path();
    let path = Path::new(&file);
    let file_name = match path.file_name() {
        Some(name) => name.to_str(),
        None => panic!("You must provide a valid file"),
    };
    let file_name = file_name.unwrap().to_string();

    let target = format!("{}/{}", project_dir, &file_name);
    // Check that file exists
    assert_eq!(
        path.exists(),
        true,
        "Input file {} does not exist",
        &file
    );
    println!("Copying file {}", &file);
    fs::copy(&file, &target)
        .expect("Could not copy source file");

    let index = project.find_change_by_file(&file_name);
    let hashed = hash_file(&target);

    // Update or add change
    if index == 9999 {
        // Add change to change list
        let change = project::Change {
            timing: timing.parse::<project::Timing>()
                .expect("Invalid timing value"),
            file: file_name.to_string(),
            hash: hashed,
            source_file: file.to_owned()
        };
        project.add_change(change);
    } else {
        let mut change = project.changes.get_mut(index).unwrap();
        change.timing = timing.parse::<project::Timing>()
            .expect("Invalid timing value");
        change.file = file_name.to_string();
        change.hash = hashed;
        println!("Replacing change with new file");
    }

    true
}

/**
 * List all changes in project 
 */
fn ls(project: &mut project::Project) -> bool {
    println!("\nCurrent changes in project:\n");
    let line = format!("|-{dash:-<10}-|-{dash:-<100}-|-{dash:-<32}-|", dash="-");
    println!("{}", line);
    println!("| {timing:10} | {file:100} | {hash:32} |", timing="Timing", file="File", hash="Hash");
    println!("{}", line);
    for change in &project.changes {
        println!("{}", change);
    }
    println!("{}\n", line);

    false
}

/**
 * Clear all changes in project 
 */
fn clear(project: &mut project::Project) -> bool {
    //TODO iterate and delete all files
    println!("Clearing all changes from project");
    project.changes.clear();

    true
}

/**
 * Create a new change in the current project
 */
fn rm(project: &mut project::Project, args: &[String]) -> bool {
    if args.len() < 2 {
        panic!("You must provide a hash to remove");
    }

    // Lookup and find matching change
    let hash = args[1].to_owned();
    let result = project.find_change_by_hash(&hash)
        .expect("No change with that hash found");

    // Remove file
    result.change.remove_file(&project);

    println!("Removing change with hash {}", hash);
    project.changes.remove(result.index);

    true
}

/**
 * Syncs all changes
 */
fn sync(project: &mut project::Project) -> bool {
    let project_dir = &project.get_path();

    for change in project.changes.iter_mut() {
        let target = format!("{}/{}", project_dir, &change.file);
        let hashed = hash_file(&change.source_file);

        // Check if file changed
        if hashed != change.hash {
            fs::copy(&change.source_file, &target)
                .expect("Could not copy source file");

            println!("Syncing change {} -> new hash {}", &change.hash, hashed);

            change.hash = hashed;
        }
    }

    true
}

/**
 * Hash a file content 
 */
fn hash_file(path: &str) -> String {
    // Open file
    let mut file = match File::open(&path) {
        Err(_) => panic!("couldn't read {}", path),
        Ok(file) => file,
    };

    // Read file contents
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Could not read input file");
    
    return format!("{:x}", md5::compute(contents));
}


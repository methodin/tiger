use bincode::{serialize, deserialize, Infinite};
use change::Change;
use config;
use getopts::Matches;
use project::Project;
use std::io::prelude::*;
use std::fs::File;
use rusoto_s3::{S3,S3Client,PutObjectRequest};
use rusoto_core::{DefaultCredentialsProvider,Region,default_tls_client};
use rusoto_credential::StaticProvider;

/**
 * Executes the package command, effectively packaging the given
 * project into a binary representation that can be uplaoded
 * elasewhere
 */
pub fn run(project: Project, matches:&Matches) {
    let config = config::load_config("package", &matches);

    // Create packaged version of project
    let mut packaged_project = Project {
        name: project.name.to_owned(),
        changes: Vec::new()
    };

    // Create packaged version of all changes, including file content
    for change in &project.changes {
        let up_content = change.read_file(&project, "up");
        let down_content = change.read_file(&project, "down");

        let packaged_change = Change {
            timing: change.timing.to_owned(),
            hash: change.hash.to_owned(),
            change_type: change.change_type.to_owned(),
            up_content: up_content,
            down_content: down_content,
        };
        packaged_project.add_change(packaged_change);
    }

    // Binary encode packaged project
    let encoded: Vec<u8> = serialize(&packaged_project, Infinite).unwrap();

    // Write to output file
    let project_dir = &project.get_path();
    let file_name = format!("{}.bin", &project.name);
    let binary_path = format!("{}/{}", &project_dir, &file_name);

    println!("Packaging project file to {}", &binary_path);

    let mut buffer = File::create(&binary_path)
        .expect("Unable to package project file");
    
    buffer.write(encoded.as_slice())
        .expect("Unable to write data to bin file");

    println!("Packaging complete");

    // Upload package to s3
    let provider = StaticProvider::new(config.s3.key, config.s3.secret, None, None);
    let region = Region::UsWest1;
    let bucket = "tiger-1234";

    let mut s3 = S3Client::new(default_tls_client().unwrap(), provider, region);
    let mut req : PutObjectRequest = Default::default();
    req.body = Some(encoded.clone());
    req.key = file_name.clone();
    req.bucket = bucket.to_string();

    match s3.put_object(&req) {
        Err(err) => println!("Failed to put object {} message: {}", &file_name, err),
        _ => println!("Successfully uploaded package to s3")
    };
}

pub fn read(path: &String) -> Project {
    // Open file
    let mut file = match File::open(&path) {
        Err(_) => panic!("couldn't read package binary {}", path),
        Ok(file) => file,
    };

    let mut buffer = Vec::new();
    // read the whole file
    file.read_to_end(&mut buffer).expect("Error");

    // Read file contents
    let project: Project = deserialize(&buffer[..]).unwrap();
    project
}

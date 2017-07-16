use bincode::{serialize, deserialize, Infinite};
use change::Change;
use config::{self,Config};
use getopts::Matches;
use project::Project;
use std::str::FromStr;
use rusoto_s3::{S3,S3Client,PutObjectRequest,GetObjectRequest};
use rusoto_core::{Region,default_tls_client};
use rusoto_credential::ChainProvider;

/**
 * Executes the package command, effectively packaging the given
 * project into a binary representation that can be uplaoded
 * elasewhere
 */
pub fn run(project: Project, args: &[String], matches:&Matches) {
    if args.len() != 1 {
        panic!("You must provide a package file name");
    }

    let file_name = args[0].replace("%", project.name.as_str());
    let file_name = format!("{}.bin", file_name);
    let config = config::load_config("package", &matches);

    println!("Packaging project file {}", &file_name);

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

    println!("Packaging complete... uploading to s3");

    // Setup s3 objects
    let provider = ChainProvider::new();
    let region = Region::from_str(config.s3.region.as_str()).unwrap();
    let bucket = config.s3.bucket;
    let s3 = S3Client::new(default_tls_client().unwrap(), provider, region);

    // Setup get object request
    let mut req : GetObjectRequest = Default::default();
    req.key = file_name.clone();
    req.bucket = bucket.to_string();
    
    // Check if object already exists
    match s3.get_object(&req) {
        Err(_) => {},
        _ => panic!("The package name you have specified already exists. Choose another e.g. %-1")
    };

    // Setup put object request
    let mut req : PutObjectRequest = Default::default();
    req.body = Some(encoded.clone());
    req.key = file_name.clone();
    req.bucket = bucket.to_string();

    // Upload package to s3
    match s3.put_object(&req) {
        Err(err) => println!("Failed to put object {} message: {}", &file_name, err),
        _ => println!("Successfully uploaded package to s3")
    };
}

/**
 * Downloads and extracts a package from s3
 */
pub fn load(file_name: &String, config: &Config) -> Project {
    // Setup s3 objects
    let provider = ChainProvider::new();
    let region = Region::from_str(config.s3.region.as_str()).unwrap();
    let bucket = config.s3.bucket.clone();
    let s3 = S3Client::new(default_tls_client().unwrap(), provider, region);

    // Setup get object request
    let mut req : GetObjectRequest = Default::default();
    req.key = format!("{}.bin", file_name.clone());
    req.bucket = bucket.to_string();
    
    // Check if object already exists
    let response = match s3.get_object(&req) {
        Err(e) => panic!(format!("Package not found or unable to connect to s3: {}", e)),
        obj => obj
    };

    // Read file contents
    let project: Project = deserialize(&response.unwrap().body.unwrap()).unwrap();
    project
}

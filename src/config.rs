use getopts::Matches;
use serde_yaml;
use std::fs::File;
use std::io::Read;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlConfig {
    pub host: String
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct S3Config {
    pub key: String,
    pub secret: String,
    pub bucket: String,
    pub region: String,
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub sql: SqlConfig,
    pub s3: S3Config,
}


pub fn load_config(command: &str, matches:&Matches) -> Config {
    // Check if config file not passed
    if !matches.opt_present("c") {
        panic!(format!("When using {} you must provide a configuration file via the -c flag", command));
    }
 
    // Get config file path
    let config = matches.opt_str("c").unwrap();
    
    // Open file
    let mut file = match File::open(&config) {
        Err(_) => panic!("couldn't read {}", config),
        Ok(file) => file,
    };

    // Read file contents
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Could not read config file");
    let config: Config = serde_yaml::from_str(&contents).unwrap();
    config
}

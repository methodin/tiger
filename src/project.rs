use std::env;
use std::fmt;
use std::fs::{self, DirBuilder};
use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::default::Default;
use std::path::Path;
use std::str::FromStr;
use serde_json;

#[derive(Clone, Serialize, Deserialize)]
pub enum Timing {
    Pre,
    Post
}
/**
 * Implement Display for Timing enum
 */
impl fmt::Display for Timing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            Timing::Pre => "pre",
            Timing::Post => "post",
        };
        write!(f, "{:10}", printable)
    }
}
/**
 * Set default for Timing
 */
impl Default for Timing {
    fn default() -> Timing { Timing::Pre }
}

/**
 * Function to map strings to Timing
 */
impl FromStr for Timing {
    type Err = ();

    fn from_str(s: &str) -> Result<Timing, ()> {
        match s {
            "pre" => Ok(Timing::Pre),
            "post" => Ok(Timing::Post),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct Change {
    pub timing: Timing,
    pub file: String,
    pub hash: String,
    #[serde(default)]
    pub source_file: String
}
/**
 * Implement Display for Change struct
 */
impl fmt::Display for Change {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "| {timing} | {file:100} | {hash:32} |",
            timing = self.timing,
            file = self.file,
            hash = self.hash)
    }
}

impl Change {
    /**
     * Remove the file from the filesystem
     */
    pub fn remove_file(self, project: &Project) {
        let project_dir = &project.get_path();
        let target = format!("{}/{}", project_dir, &self.file);

        fs::remove_file(target)
            .expect("Could not remove file");
    }
}

#[derive(Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub changes: Vec<Change>
}

impl Project{
    /**
     * Add a change to the internal list
     */
    pub fn add_change(&mut self, change: Change) {
        self.changes.push(change.clone());
    }   

    /**
     * Return file path to project
     */
    pub fn get_path(&self) -> String {
        let dir = env::current_dir().unwrap();
        return format!("{}/tiger/{}", dir.display(), &self.name);
    }

    /**
     * Find a change by hash
     */
    pub fn find_change_by_hash(&self, hash: &str) -> Option<SearchResult> {
        let mut changes : Vec<SearchResult> = Vec::new();
        for (i, change) in self.changes.iter().enumerate() {
            if change.hash.starts_with(hash) {
                changes.push(SearchResult {
                    change: change.to_owned(),
                    index: i
                });
            }
        }

        // Make sure we don't get more than one
        assert!(changes.len() < 2, "The hash provided matched more than one change - please reduce the scope of your hash");

        if changes.len() == 1 {
            changes.pop()
        } else {
            None
        }
    }

    /**
     * Find a change by file
     */
    pub fn find_change_by_file(&self, file_name: &str) -> usize {
        let mut iter = self.changes.iter();
        match iter.position(|&ref x| x.file == file_name) {
            Some(index) => index,
            None => 9999,
        }
    }
}

pub struct SearchResult {
    pub change: Change,
    pub index: usize
}

const PROJECT_FILE: &str = "project.json";

/**
 * Create a new project in the current directory
 * Will dump a project json file in the directory specificed
 */
pub fn create(args: &[String]) {
	let dir = env::current_dir().unwrap();

    let project_dir = format!("{}/tiger/{}", dir.display(), args[0]);

    // Make sure project doesn't already exist
    if let Ok(_) = fs::metadata(&project_dir) {
        println!("Project {} already exists", args[0]);
        return;
    }

    // Create new dir
    DirBuilder::new()
        .recursive(true)
        .create(&project_dir).unwrap();

    println!("Creating project {} in current dir: {}", args[0], dir.display());

    // Create project instance
    let project = Project {
        name: args[0].to_owned(),
        changes: Vec::new()
    };

    save(&project);
}

/**
 * Load an existing project from a project file
 */
pub fn load(project: &str) -> Project {
	let dir = env::current_dir().unwrap();

    // Set path
    let yaml_path = format!("{}/tiger/{}/{}", dir.display(), &project, PROJECT_FILE);

    // Open file
    let mut file = match File::open(&yaml_path) {
        Err(why) => panic!("couldn't create {}: {}", yaml_path, why.description()),
        Ok(file) => file,
    };

    // Read file contents
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Could not read project file");

    // Parse YAML data
    let project: Project = match serde_json::from_str(&contents) {
        Ok(project) => project,
        Err(e) => panic!("Could not load project json file {:?}", e),
    };

    project
}

/**
 * Save a project and write it out
 */
pub fn save(project: &Project) {
    let project_dir = project.get_path();

    // Prepare project file path
    let project_path = format!("{}/{}", project_dir, PROJECT_FILE);
    let path = Path::new(&project_path);
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };

    // Encode project as a json string
    let content = match serde_json::to_string(&project) {
        Ok(res) => res,
        Err(_) => panic!("Could not serialize project file"),
    };

    // Write the string to the project file
    match file.write_all(content.as_bytes()) {
        Err(why) => panic!("Error: Couldn't write to {}: {}", display, why.description()),
        Ok(_) => println!("Successfully created project file {}", display),
    }
}

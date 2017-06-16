use change::Change;
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

#[derive(PartialEq, Clone, Serialize, Deserialize)]
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
     * Save a project and write it out
     */
    pub fn save(&self) {
        let project_dir = self.get_path();

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
        let content = match serde_json::to_string(&self) {
            Ok(res) => res,
            Err(_) => panic!("Could not serialize project file"),
        };

        // Write the string to the project file
        match file.write_all(content.as_bytes()) {
            Err(why) => panic!("Error: Couldn't write to {}: {}", display, why.description()),
            Ok(_) => println!("Successfully created project file {}", display),
        }
    }

    /**
     * Create a new project in the current directory
     * Will dump a project json file in the directory specificed
     */
    pub fn create(name: &String) {
        let dir = env::current_dir().unwrap();

        let project_dir = format!("{}/tiger/{}", dir.display(), name);

        // Make sure project doesn't already exist
        if let Ok(_) = fs::metadata(&project_dir) {
            println!("Project {} already exists", name);
            return;
        }

        // Create new dir
        DirBuilder::new()
            .recursive(true)
            .create(&project_dir).unwrap();

        println!("Creating project {} in current dir: {}", name, dir.display());

        // Create project instance
        let project = Project {
            name: name.to_owned(),
            changes: Vec::new()
        };

        project.save();
    }

    /**
     * Clear all changes in project 
     */
    pub fn clear(&mut self) {
        println!("Clearing all changes from project");

        let project_dir = &self.get_path();

        for change in &self.changes {
            let change_dir = format!("{}/{}", &project_dir, change.hash);
            fs::remove_dir_all(&change_dir)
                .expect(format!("Could not remove dir {}", &change_dir).as_str());
            println!("Removed hash {}", change.hash);
        }

        self.changes.clear();
        self.save();
    }

    /**
     * List all changes in project 
     */
    pub fn ls(&mut self) {
        println!("> Current changes in project:\n");
        let line = format!("|-{dash:-<10}-|-{dash:-<10}-|-{dash:-<32}-|", dash="-");
        println!("{}", line);
        println!("| {timing:10} | {change_type:10} | {hash:32} |", timing="Timing", change_type="Type", hash="Hash");
        println!("{}", line);
        for change in &self.changes {
            println!("{}", change);
        }
        println!("{}\n", line);
    }
}

pub struct SearchResult {
    pub change: Change,
    pub index: usize
}

const PROJECT_FILE: &str = "project.json";

/**
 * Load an existing project from a project file
 */
pub fn load(project: &str) -> Project {
	let dir = env::current_dir().unwrap();

    // Set path
    let yaml_path = format!("{}/tiger/{}/{}", dir.display(), &project, PROJECT_FILE);

    // Open file
    let mut file = match File::open(&yaml_path) {
        Err(why) => panic!("Couldn't open file {}: {}", yaml_path, why.description()),
        Ok(file) => file,
    };

    // Read file contents
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Could not read project file");

    // Parse YAML ata
    let project: Project = match serde_json::from_str(&contents) {
        Ok(project) => project,
        Err(e) => panic!("Could not load project json file {:?}", e),
    };

    project
}


use project::{Project,Timing};
use md5;
use rand::{self,Rng};
use std::fmt;
use std::fs::{self,File,DirBuilder};
use std::io::prelude::*;
use std::str::FromStr;
use std::process::Command;

#[derive(Clone, Serialize, Deserialize)]
pub enum ChangeType {
    Sql
}
impl Default for ChangeType {
    fn default() -> ChangeType { ChangeType::Sql }
}
impl fmt::Display for ChangeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            ChangeType::Sql => "sql"
        };
        write!(f, "{:10}", printable)
    }
}
impl FromStr for ChangeType {
    type Err = ();

    fn from_str(s: &str) -> Result<ChangeType, ()> {
        match s {
            "sql" => Ok(ChangeType::Sql),
            _ => Err(()),
        }
    }
}

/**
 * The change struct
 */
#[derive(Clone, Serialize, Deserialize, Default)]
pub struct Change {
    pub timing: Timing,
    #[serde(default)]
    pub change_type: ChangeType,
    pub hash: String,
}

/**
 * Implement Display for Change struct
 */
impl fmt::Display for Change {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "| {timing:10} | {change_type:10} | {hash:32} |",
            timing = self.timing,
            change_type = self.change_type,
            hash = self.hash)
    }
}

impl Change {
    /**
     * Read file contents
     */
    pub fn read_file(&self, project: &Project, direction: &str) -> String {
        let project_dir = &project.get_path();
        let target = format!("{}/{}/{}.sql",
            project_dir, &self.hash, &direction);

        // Open file
        let mut file = match File::open(&target) {
            Err(_) => panic!("couldn't read {}", target),
            Ok(file) => file,
        };

        // Read file contents
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Could not read input file");

        contents
    }
}

/**
 * Create a new change
 */
pub fn add(project: &mut Project, timing: &str, args: &[String]) {
    if args.len() != 1 {
        panic!("You must provide a change type");
    }

    let change_type = args[0].to_owned();

    // Create hash and dir
    let mut rng = rand::thread_rng();
    let rnd = format!("{}", rng.gen::<u32>());
    let hash = format!("{:x}", md5::compute(rnd));
   
    // Create new change dir
    let project_dir = &project.get_path();
    let change_dir = format!("{}/{}", &project_dir, hash);
    DirBuilder::new()
        .create(&change_dir).unwrap();

    println!("Creating new change {}", &hash);

    // Create up file
    File::create(format!("{}/{}", &change_dir, "up.sql"))
        .expect("Could not create up file");

    println!("Creating new up file {}/{}", &change_dir, "up.sql");

    // Create down file
    File::create(format!("{}/{}", &change_dir, "down.sql"))
        .expect("Could not create down file");

    println!("Creating new up file {}/{}", &change_dir, "down.sql");
        
    // Add change to change list
    let change = Change {
        timing: timing.parse::<Timing>()
            .expect("Invalid timing value"),
        hash: hash,
        change_type: change_type.parse::<ChangeType>()
            .expect("Invalid change type value"),
    };
    project.add_change(change);
    project.save();
}

/**
 * Executes the rm command
 */
pub fn rm(project: &mut Project, args: &[String]) {
    if args.len() != 1 {
        panic!("You must provide a hash to remove");
    }

    // Lookup and find matching change
    let hash = args[0].to_owned();
    let result = project.find_change_by_hash(&hash)
        .expect("No change with that hash found");

    // Remove file
    let project_dir = &project.get_path();
    let change_dir = format!("{}/{}", &project_dir, result.change.hash);
    fs::remove_dir_all(&change_dir)
        .expect(format!("Could not remove dir {}", &change_dir).as_str());

    println!("Removing change with hash {}", result.change.hash);
    project.changes.remove(result.index);
    project.save();
}

/**
 * Executes the edit command
 */
pub fn edit(project: &mut Project, args: &[String]) {
    if args.len() != 1 {
        panic!("You must provide a hash to edit");
    }

    // Lookup and find matching change
    let hash = args[0].to_owned();
    let result = project.find_change_by_hash(&hash)
        .expect("No change with that hash found");

    // Remove file
    let project_dir = &project.get_path();
    let change_dir = format!("{}/{}", &project_dir, result.change.hash);

    let up_file = format!("{}/up.sql", change_dir);
    let down_file = format!("{}/down.sql", change_dir);

    Command::new("vim")
        .arg(&up_file)
        .arg(&down_file)
        .status()
        .expect("Failed to execute process");
}

use std::env;
mod new_project;

const VERSION: &str = "0.1.0";
pub const SUPPORTED_MINECRAFT_VERSIONS: [&str; 2] = [
    "1.18.2",
    "1.18.1"
];

pub struct ProjectData {
    name: String,
    description: String,
    minecraft_version: String,
    project_path: String
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("amogus: {}", args[0]);

    println!("Dyphae Language Version {}\n-------------------------------------", VERSION);

    process_args(args)
}

fn process_args(args: Vec<String>) {
    if args[1] == "new" {
        println!("Creating new project with name '{}'", args[2]);

        let mut allow_version: bool = false;
        for version in SUPPORTED_MINECRAFT_VERSIONS {
            if args[3] == version {
                allow_version = true;
            }
        }

        if allow_version {
            let project_data: ProjectData = ProjectData {
                name: args[2].to_string(),
                description: args[4].to_string(),
                minecraft_version: args[3].to_string(),
                project_path: args[5].to_string()
            };
            new_project::gen(project_data)
        } else {
            println!("Invalid minecraft version, please choose a supported version")
        }
    } else {
        println!("Unknown command: '{}'", args[1]);
    }
}
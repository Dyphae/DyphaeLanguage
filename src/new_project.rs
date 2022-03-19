use std::fs;
use std::fs::File;
use std::io::prelude::*;
use crate::ProjectData;

pub fn gen(project: ProjectData) -> std::io::Result<()> {
    println!("{}, {}, {}", project.name, project.description, project.minecraft_version);
    
    // Create project directory
    let proj_path = format!("{}\\{}", project.project_path, project.name);
    let _result = fs::create_dir(&proj_path);

    // Create project files
    let _result = fs::create_dir(format!("{}\\data", &proj_path));

    let pack_namespace_path = format!("{}\\data\\{}", &proj_path, project.namespace);
    let _result = fs::create_dir(&pack_namespace_path);
    let _result = fs::create_dir(format!("{}\\functions", &pack_namespace_path));
    let mut file = File::create(format!("{}\\functions\\load.dscript", &pack_namespace_path))?;
    file.write(b"say Hello World!")?;
    let mut file = File::create(format!("{}\\functions\\tick.dscript", &pack_namespace_path))?;
    file.write(b"give @a minecraft:diamond 1")?;

    let minecraft_namespace_path = format!("{}\\data\\minecraft", &proj_path);
    let _result = fs::create_dir(&minecraft_namespace_path);
    let _result = fs::create_dir(format!("{}\\tags", &minecraft_namespace_path));
    let _result = fs::create_dir(format!("{}\\tags\\functions", &minecraft_namespace_path));
    let mut file = File::create(format!("{}\\tags\\functions\\load.json", &minecraft_namespace_path))?;
    file.write(format!("{{\"values\":[\"{}:load\"]}}", project.namespace).as_bytes())?;
    let mut file = File::create(format!("{}\\tags\\functions\\tick.json", &minecraft_namespace_path))?;
    file.write(format!("{{\"values\":[\"{}:tick\"]}}", project.namespace).as_bytes())?;

    Ok(())
}
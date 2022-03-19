use crate::ProjectData;

pub fn gen(project: ProjectData) {
    println!("{}, {}, {}", project.name, project.description, project.minecraft_version)
}
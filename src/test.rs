use super::*;
use std::fs;
use std::path::Path;

fn delete_directory_if_exists(dir_path: &str) -> std::io::Result<()> {
    let path = Path::new(dir_path);
    if path.exists() {
        fs::remove_dir_all(path)?;
    }
    Ok(())
}
#[test]
fn test_create_standalone_success() {

    let project_name = "test-project";
    match delete_directory_if_exists("relative_path_to_directory") {
        Ok(_) => println!("Directorio borrado o no existía."),
        Err(e) => eprintln!("Error al borrar el directorio: {}", e),
    }

    let result = create_standalone(project_name);
    println!("{:?}", result);
}

mod custom_logger;
mod utils;

#[cfg(test)]
mod test;

use custom_logger::init_logger;
use std::{env, io, process::{Command, Stdio}};
use std::io::Write;
use utils::{format_project_name, read_error_message};

fn main() {
    init_logger();
    if let Err(e) = run_flow() {
        error!("An error occurred: {}", e);
    }

}

fn run_flow() -> Result<(), Box<dyn std::error::Error>> {
    let is_unix = is_unix_os();
    let python_cmd = if is_unix { "python3" } else { "python" };

    info!("Is Unix: {}", is_unix);
    info!("Using '{}' as python command", python_cmd);

    check_python_installed(python_cmd)?;
    let project_name = get_project_name()?;
    check_and_install_package("kedro")?;
    create_standalone(&project_name)?;
    Ok(())
}

fn is_unix_os() -> bool {
    matches!(env::consts::OS, "linux" | "macos")
}

fn check_python_installed(python_cmd: &str) -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new(python_cmd).arg("--version").output()?;
    if !output.status.success() {
        return Err("Python is not installed.".into());
    }

    let env_path = Command::new(python_cmd)
        .arg("-c")
        .arg("import sys; print(sys.prefix)")
        .output()?;

    let env_path_str = String::from_utf8_lossy(&env_path.stdout).trim().to_string();
    info!("The environment is: {}", env_path_str);
    Ok(())
}

fn get_project_name() -> Result<String, Box<dyn std::error::Error>> {
    input!("Please enter the project name (package):");
    let mut project_name = String::new();
    io::stdin().read_line(&mut project_name)?;
    let formatted_name = format_project_name(project_name.trim()).to_lowercase();
    if !formatted_name.is_empty() {
        answer!("The project will be: {}", formatted_name);
        Ok(formatted_name)
    } else {
        Err("Project name can't be empty".into())
    }
}

fn check_and_install_package(package: &str) -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("pip").arg("show").arg(package).output()?;
    if output.stdout.is_empty() {
        info!("The '{}' package is not installed. Installing...", package);
        let status = Command::new("pip").arg("install").arg(package).status()?;
        if status.success() {
            info!("The '{}' package has been successfully installed.", package);
        } else {
            return Err(format!(
                "An error occurred while trying to install the '{}'.",
                package
            )
            .into());
        }
    } else {
        info!("The '{}' package is already installed.", package);
    }
    Ok(())
}


fn create_standalone(project_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut output = Command::new("kedro")
        .arg("new")
        .arg("--starter=standalone-datacatalog")
        .stdin(Stdio::piped())
        .spawn()?;

    if let Some(stdin) = output.stdin.as_mut() {
        stdin.write_all(format!("{}\n", project_name).as_bytes())?;
    } else {
        let error_message = read_error_message(&mut output)?;
        return Err(format!("Failed to open stdin: {}", error_message).into());
    }

    let status = output.wait()?;

    if status.success() {
        Ok(())
    } else {
        let error_message = read_error_message(&mut output)?;
        Err(format!("Can't create Kedro project: {}", error_message).into())
    }

}


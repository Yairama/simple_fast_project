mod custom_logger;
mod utils;

#[cfg(test)]
mod test;

use custom_logger::init_logger;
use std::error::Error;
use std::io::Write;
use std::{
    env, fs, io,
    process::{Command, Stdio},
};
use utils::{format_project_name, read_error_message};

fn main() {
    init_logger();
    if let Err(e) = run_flow() {
        error!("An error occurred: {}", e);
    }
}

fn run_flow() -> Result<(), Box<dyn Error>> {
    let is_unix = is_unix_os();
    let python_cmd = if is_unix { "python3" } else { "python" };

    info!("Is Unix: {}", is_unix);
    info!("Using '{}' as python command", python_cmd);

    check_python_installed(python_cmd)?;
    let project_name = get_project_name()?;

    //TODO add an exception cather in order to not interrupt the flow in case of not installed package

    let packages = vec![
        "numpy",
        "pandas",
        "matplotlib",
        "seaborn",
        "plotly",
        "openpyxl",
        "ipykernel",
        "jupyter",
        "jupyterlab",
    ];

    let missing_packages = check_installed_packages(&packages)?;

    for package in &missing_packages {
        install_package(package)?;
    }

    good!("All recommended packages are now installed!!");

    create_standalone(&project_name)?;
    create_additional_folders(&project_name)?;
    Ok(())
}

fn is_unix_os() -> bool {
    matches!(env::consts::OS, "linux" | "macos")
}

fn check_python_installed(python_cmd: &str) -> Result<(), Box<dyn Error>> {
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

fn get_project_name() -> Result<String, Box<dyn Error>> {
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

fn check_installed_packages<'a>(packages: &'a [&'a str]) -> Result<Vec<&'a str>, Box<dyn Error>> {
    let output = Command::new("pip").arg("list").output()?;
    let installed = String::from_utf8(output.stdout)?;

    let mut missing_packages = vec![];
    for &package in packages {
        if !installed.contains(package) {
            info!("The '{}' package is not installed.", package);
            missing_packages.push(package);
        } else {
            info!("The '{}' package is already installed.", package);
        }
    }

    Ok(missing_packages)
}

fn install_package(package: &str) -> Result<(), Box<dyn Error>> {
    println!("Installing '{}'", package);
    let status = Command::new("pip").arg("install").arg(package).status()?;
    if status.success() {
        println!("The '{}' package has been successfully installed.", package);
    } else {
        return Err(format!(
            "An error occurred while trying to install the '{}'.",
            package
        )
        .into());
    }
    Ok(())
}

fn create_standalone(project_name: &str) -> Result<(), Box<dyn Error>> {
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
        good!(
            "The project folder '{}' was created in the current path",
            project_name
        );
        Ok(())
    } else {
        let error_message = read_error_message(&mut output)?;
        Err(format!("Can't create Kedro project: {}", error_message).into())
    }
}

fn create_additional_folders(project_name: &str) -> Result<(), Box<dyn Error>> {
    info!("Creating additional resources");
    create_folder(&format!("{}/src", project_name))?;
    create_folder(&format!("{}/resources", project_name))?;

    Ok(())
}

fn create_folder(path: &str) -> Result<(), Box<dyn Error>> {
    let status = fs::create_dir(path);

    if status.is_ok() {
        Ok(())
    } else {
        let error_message = format!("Can't create the folder: {}", status.unwrap_err());
        Err(error_message.into())
    }
}

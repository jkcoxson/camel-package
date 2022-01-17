// jkcoxson

use crate::ui;
use serde::{Deserialize, Serialize};
use std::io::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    name: String,
    install_path: String,
    license: String,
    pub platforms: Vec<Platform>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Platform {
    pub arch: String,
    pub libs: Vec<Lib>,
    pub bins: Vec<Binary>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Lib {
    pub name: String,
    pub path: String,
    pub install_path: String,
    pub variable_path: bool,
    pub optional: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Binary {
    pub name: String,
    pub path: String,
    pub optional: bool,
}

impl Config {
    pub fn load(path: &str) -> Result<Config, Error> {
        let file = std::fs::read_to_string(path)?;
        let config: Config = serde_json::from_str(&file)?;
        Ok(config)
    }
    pub fn new() -> Self {
        // Get a name for the package
        let name = ui::text_prompt("What is the name of your package?");
        let install_path = ui::text_prompt("Where would you like the binary installed to?");
        let license;
        if ui::yes_or_no("Would you like to include a license?") {
            // Get the license text
            license = ui::text_prompt("Enter the license text:");
        } else {
            license = String::from("");
        }

        let mut config = Config {
            name,
            install_path,
            license,
            platforms: vec![],
        };

        // Get the platforms available
        let output = std::process::Command::new("rustc")
            .arg("--print")
            .arg("target-list")
            .output()
            .unwrap();
        let stdout = String::from_utf8(output.stdout).unwrap();
        let mut platforms = Vec::new();
        for line in stdout.lines() {
            platforms.push(line.to_string());
        }
        platforms.push("Cancel".to_string());
        loop {
            // Get a platform to configure
            let platform = ui::multi_prompt("Select a platform to configure:", platforms.clone());
            if platform == "Cancel" {
                break;
            }

            let mut platform = Platform {
                arch: platform.clone(),
                libs: vec![],
                bins: vec![],
            };

            // Get libraries
            loop {
                if ui::yes_or_no("Add a library?") {
                    // Get the name of the library
                    let name = ui::text_prompt("Enter the name of the library:");
                    // Get the path to the library
                    let path = ui::text_prompt("Enter the path to the library:");
                    // Get the install path for the library
                    let install_path = ui::text_prompt("Enter the install path for the library:");
                    // Determine if the path is a variable
                    let variable_path = ui::yes_or_no("Is the path ");
                    // Determine if the library is optional
                    let optional = ui::yes_or_no("Is the library optional?");
                    // Add the library to the platform
                    platform.libs.push(Lib {
                        name,
                        path,
                        install_path,
                        variable_path,
                        optional,
                    });
                } else {
                    break;
                }
            }

            // Get binaries
            loop {
                if ui::yes_or_no("Add a binary to run?") {
                    // Get the name of the binary
                    let name = ui::text_prompt("Enter the name of the binary:");
                    // Get the path to the binary
                    let path = ui::text_prompt("Enter the path to the binary:");
                    // Determine if the binary is optional
                    let optional = ui::yes_or_no("Is the binary optional?");
                    // Add the binary to the platform
                    platform.bins.push(Binary {
                        name,
                        path,
                        optional,
                    });
                } else {
                    break;
                }
            }

            // Add the platform to the config
            config.platforms.push(platform);
        }

        config
    }
}

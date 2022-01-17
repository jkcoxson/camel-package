// jkcoxson

mod config;
mod ui;

fn main() {
    // Detect if there is a Cargo.toml file in the current directory
    let toml = std::fs::read("Cargo.toml").unwrap_or_else(|_| {
        println!("Run this command from the root of your project.");
        std::process::exit(1);
    });

    // Save the current directory
    let root_dir = std::env::current_dir().unwrap();

    // Determine if we need to reconfigure the build
    let mut args: Vec<String> = std::env::args().collect();
    let mut reconfigure = false;
    for i in 1..args.len() {
        if args[i] == "--reconfigure" {
            reconfigure = true;
            args.remove(i);
        }
    }

    // Get the name of the crate from the Cargo.toml file
    let name = std::str::from_utf8(&toml)
        .unwrap()
        .lines()
        .find(|line| line.starts_with("name = "))
        .unwrap()
        .split("\"")
        .nth(1)
        .unwrap();
    println!("Building installer for {}", name);

    // Determine if the CamelPackage folder exists
    let camel_package_dir = std::env::current_dir().unwrap().join("CamelPackage");
    if !camel_package_dir.exists() {
        std::fs::create_dir(camel_package_dir.clone()).unwrap();
    }

    // Determine if the CamelPackage/config.json file exists
    let config_file = camel_package_dir.join("config.json");
    let config;
    if !config_file.exists() || reconfigure {
        // Create the CamelPackage/config.json file
        config = config::Config::new();
        let config_json = serde_json::to_string_pretty(&config).unwrap();
        std::fs::write(config_file, config_json).unwrap();
    } else {
        // Load the CamelPackage/config.json file
        config = config::Config::load(config_file.as_path().to_str().unwrap()).unwrap();
    }

    // Determine if the CamelPackage/output folder exists
    let output_dir = camel_package_dir.join("output");
    if !output_dir.exists() {
        std::fs::create_dir(output_dir.clone()).unwrap();
    }

    // Include the template rs file as a string
    let template = include_str!("template.rs").to_string();
    let template = template.replace(
        "camel_insert_package_name!()",
        &format!("\"{}\".to_string()", name).to_string(),
    );

    for platform in config.platforms {
        // Run the command to build the binary
        let command = std::process::Command::new("cargo")
            .arg("build")
            .arg("--target")
            .arg(&platform.arch.clone())
            .arg("--release")
            .output()
            .unwrap();
        if !command.status.success() {
            println!("Failed to build the binary for {}", platform.arch);
            continue;
        }

        let template = template.replace(
            "camel_insert_license!()",
            &format!("\"{}\"", &config.license).to_string(),
        );
        let template = template.replace(
            "camel_insert_install_path!()",
            &format!("\"{}\"", &platform.install_path).to_string(),
        );

        // Get the full path to the binary
        let binary = std::env::current_dir()
            .unwrap()
            .join("target")
            .join(&platform.arch)
            .join("release")
            .join(&format!("{}", name));

        let template = template.replace(
            "camel_insert_binary!()",
            &format!("include_bytes!(\"{}\")", binary.as_path().to_str().unwrap()),
        );

        // Create CamelPackage/platform to build the installer for
        let platform_dir = camel_package_dir.join(&platform.arch);
        if !platform_dir.exists() {
            std::fs::create_dir(platform_dir.clone()).unwrap();
        }
        // Change current directory to CamelPackage/platform
        std::env::set_current_dir(platform_dir.clone()).unwrap();
        // Run cargo init
        std::process::Command::new("cargo")
            .arg("init")
            .arg("--bin")
            .arg(&name)
            .output()
            .unwrap();
        // Replace main.rs with the template
        std::fs::write(format!("{}/src/main.rs", name), template).unwrap();
        // Change current directory to the name
        std::env::set_current_dir(platform_dir.join(name)).unwrap();
        // Add cursive to the Cargo.toml file
        let mut toml = std::fs::read_to_string("Cargo.toml").unwrap();
        if !toml.contains("cursive") {
            toml = format!("{}\ncursive = \"*\"", toml);
            std::fs::write("Cargo.toml", toml).unwrap();
        }
        // Run cargo build
        std::process::Command::new("cargo")
            .arg("build")
            .arg("--target")
            .arg(&platform.arch)
            .arg("--release")
            .output()
            .unwrap();
        // Attempt to move the installer to the output directory
        match std::fs::copy(
            format!("target/{}/release/{}", platform.arch, name),
            output_dir.join(&format!("{}-installer-{}", name, platform.arch)),
        ) {
            Ok(_) => {}
            Err(e) => {
                println!("Failed to build the installer for {}: {}", platform.arch, e);
                continue;
            }
        }
        // Change current directory back to the root directory
        std::env::set_current_dir(&root_dir).unwrap();
        // Remove CamelPackage/platform
        std::fs::remove_dir_all(platform_dir).unwrap();
    }
}

// jkcoxson

fn main() {
    // Detect if there is a Cargo.toml file in the current directory
    let toml = std::fs::read("Cargo.toml").unwrap_or_else(|_| {
        println!("Run this command from the root of your project.");
        std::process::exit(1);
    });

    // Determine if we need to reconfigure the build
    let args: Vec<String> = std::env::args().collect();
    let mut reconfigure = false;
    for i in 1..args.len() {
        if args[i] == "--reconfigure" {
            reconfigure = true;
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
    if camel_package_dir.exists() {
    } else {
        // Create the CamelPackage folder
        std::fs::create_dir(camel_package_dir).unwrap();
    }

    // Include the template rs file as a string
    let template = include_str!("template.rs").to_string();
    let template = template.replace(
        "camel_insert_package_name!()",
        &format!("\"{}\"", name).to_string(),
    );
    println!("{}", template);
}

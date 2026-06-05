
fn main() {
    let path = match std::env::current_dir() {
        Ok(actual_path) => actual_path,
        Err(e) => {
            eprintln!("Failed to get current directory: {}", e);
            return;
        }
    };

    let args: Vec<String> =std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Error: Missing arguments. Usage: yorukit DO_LATER");
        return
    }
    
    let command = &args [1];

    match command.as_str() {
        "init" => {
            if args.len() < 3 {
                eprintln!("Error: Missing project name. Usage: yorukit init <name>");
                return;
            }
            let name = &args[2];

            println!("Creating project under {}/{}", path.display(), name);
            match std::fs::create_dir(name) {
                Ok(_) => println!("Project created under {}/{}", path.display(), name),
                Err(e) => eprintln!("Failed to create project: {}", e),
            }
        }
        "build" => {
            println!("DO_LATER");
        }
        _ => {
            eprintln!("Error: Unknown command, {}, please use DO_LATER", command);
        }
    }
}
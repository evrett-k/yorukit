mod creator;

fn main() {
    let args: Vec<String> =std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Error: Missing arguments. Usage: yorukit DO_LATER");
        return
    }
    
    let command = &args [1];

    match command.as_str() {
        // DO_LATER
        "build" => {
            println!("DO_LATER");
        }
        // creator.rs -- creator
        "init" => {
            creator::creator();
        }
        _ => {
            eprintln!("Error: Unknown command, {}, please use DO_LATER", command);
        }
    }
}
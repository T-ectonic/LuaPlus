use colored::Colorize;

const FILE_TYPES: [&str; 3] = ["luap", "luaplus", "lua+"];
const COMPILER_VERSION: &str = "0.1";
const LANGUAGE_VERSION: &str = "0.1";
const LUA_VERSION: &str = "5.4.4";
const BANNER: &str = r#" _                   _____  _
| |                 |  __ \| |
| |    _   _  __ _  | |__) | |_   _ ___
| |   | | | |/ _` | |  ___/| | | | / __|
| |___| |_| | (_| | | |    | | |_| \__ \
|______\__,_|\__,_| |_|    |_|\__,_|___/"#;

fn main() {
    println!("{}", BANNER.blue());
    println!("Compiler Version: {}", COMPILER_VERSION.green());
    println!("Language Version: {}", LANGUAGE_VERSION.yellow());
    println!("Lua Version: {}", LUA_VERSION.yellow());
    println!(
        "{} {}",
        "Supported File Types:".green(),
        FILE_TYPES.join(", ").yellow()
    );

    loop {
        println!("\nType help to see the commands available");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim().to_lowercase();

        match input.as_str() {
            "help" => println!(":skull:"),
            "documentation" | "doc" => {
                println!("Documentation: https://pcs-personal-organization.gitbook.io/lua+/")
            }
            "compile" | "build" => todo!("Implement compile command"),
            "exit" | "quit" => break,
            _ => println!("{}", "Unknown command".red()),
        }
    }
}

use std::env;

use tenkan::{
    ENV_FILE_DEFAULT_NAME, TENKAN_DEFAULT_HEADER, create_c_file, print_help, print_version,
    read_env_file,
};

fn main() {
    let mut args = env::args();
    let header_name = args.nth(1).unwrap_or(TENKAN_DEFAULT_HEADER.to_owned());
    if header_name.trim() == "version" || header_name.trim() == "v" {
        print_version();
        return;
    }
    match header_name.trim() {
        "version" | "v" => print_version(),
        "help" | "h" => print_help(),
        _ => {
            let path = args.nth(2).unwrap_or(ENV_FILE_DEFAULT_NAME.to_owned());
            let env_map = read_env_file(&path).unwrap();
            match create_c_file(&header_name, &env_map) {
                Ok(_) => println!("✅ Wrote header {} succesfully!", header_name),
                Err(e) => eprintln!("⛔ Failed to write header file :(, err {}", e),
            }
        }
    }
}

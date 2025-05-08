use std::{
    collections::HashMap,
    fs::{File, OpenOptions, write},
    io::{BufRead, BufReader, Error, Write},
    path::Path,
};

pub const TENKAN_DEFAULT_HEADER: &'static str = "tenkan.h";
pub const ENV_FILE_DEFAULT_NAME: &'static str = ".env";

/// This function exits if provided .env file is invalid!
pub fn read_env_file<P: AsRef<Path>>(file_name: P) -> Result<HashMap<String, String>, Error> {
    let f = File::open(file_name)?;
    let f = BufReader::new(f);

    let env_map = f
        .lines()
        .map(|line| {
            let l = line.unwrap().trim().to_owned();
            let (key, value) = l.split_once('=').unwrap_or_else(|| {
                eprintln!("Invalid env var, exiting...");
                std::process::exit(1);
            });
            Ok((key.to_owned(), value.to_owned()))
        })
        .collect();

    env_map
}
/// Returns error if file creating or writing goes wrong
pub fn create_c_file<P: AsRef<Path>>(
    file_name: P,
    env_map: &HashMap<String, String>,
) -> Result<(), Error> {
    let mut f = OpenOptions::new()
        .write(true)
        .create(true)
        .open(file_name)?;
    for (key, value) in env_map {
        let env_var = format!("const char* {} = \"{}\";\n", key, value);
        f.write_all(env_var.as_bytes())?;
    }
    Ok(())
}

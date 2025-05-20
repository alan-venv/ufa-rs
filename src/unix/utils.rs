use std::{
    env,
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Write},
    os::unix::fs::MetadataExt,
    path::{Path, PathBuf},
};

use crate::{bash, logger::custom::CustomStr};

use super::models::EntryPoint;

pub fn verify_root() {
    if let Ok(val) = std::fs::metadata("/proc/self").map(|m| m.uid()) {
        if val != 0 {
            println!("User isn't root");
            std::process::exit(1);
        }
    }
}

pub fn create_entrypoint(mut content: EntryPoint) {
    let user = match env::var("USER") {
        Ok(value) => value,
        Err(_) => {
            println!("{}", "Could not get the user name".red());
            std::process::exit(0);
        }
    };
    let entrypoint_path = &format!("/home/{}/.local/share/applications", user);
    let entrypoint_file = &format!(
        "{}/{}.desktop",
        entrypoint_path,
        content.name.replace(" ", "")
    );
    bash::exec("mkdir", &["-p", &entrypoint_path]);
    let file_path = Path::new(entrypoint_file);
    let mut file = File::create(file_path).unwrap();
    file.write_all(content.get_content().as_bytes()).unwrap();
    bash::exec("chmod", &["+x", &entrypoint_file]);
}

pub fn get_bashrc_path() -> PathBuf {
    let user = match env::var("USER") {
        Ok(value) => value,
        Err(_) => {
            println!("{}", "Could not get the user name".red());
            std::process::exit(0);
        }
    };
    let path = format!("/home/{}/.bashrc", user);
    return PathBuf::from(path);
}

pub fn add_to_bashrc(lines: &[&str]) {
    let mut file = OpenOptions::new()
        .append(true)
        .open(get_bashrc_path())
        .unwrap();

    writeln!(file, "").unwrap();
    for line in lines {
        writeln!(file, "{}", line).unwrap();
    }
}

pub fn remove_from_bashrc(lines: &[&str]) {
    let file_path = get_bashrc_path();
    let path = file_path.as_path();
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)
        .unwrap();

    let reader = BufReader::new(&file);
    let mut new_lines: Vec<String> = Vec::new();

    // Collect all lines except those containing the search string
    for line in reader.lines() {
        let line = line.unwrap();
        if !lines.contains(&line.as_str()) {
            new_lines.push(line);
        }
    }

    // Reopen the file in write mode and overwrite it with the filtered lines
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path)
        .unwrap();

    for line in new_lines {
        writeln!(file, "{}", line).unwrap();
    }
}

pub fn replace_from_bashrc(lines: &[&str]) {
    remove_from_bashrc(lines);
    add_to_bashrc(lines);
}

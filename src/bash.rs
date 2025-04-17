use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::thread;

use crate::logger::custom::CustomStr;

pub fn exec(command: &str) {
    Command::new("bash")
        .arg("-c")
        .arg(command)
        .status()
        .expect("Error while invoking bash command");
}

pub fn capture(command: &str) -> String {
    let output = Command::new("bash")
        .arg("-c")
        .arg(command)
        .output()
        .expect("Error while invoking bash command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    format!("{}{}", stdout, stderr)
}

pub fn spawn(command: &str) -> String {
    let mut child = Command::new("bash")
        .arg("-c")
        .arg(command)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Error while invoking bash command");

    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();

    let stdout_thread = thread::spawn(move || {
        let reader = BufReader::new(stdout);
        let mut out = String::new();
        for line in reader.lines() {
            if let Ok(line) = line {
                println!("{}", line.green());
                out.push_str(&line);
                out.push('\n');
            }
        }
        out
    });

    let stderr_thread = thread::spawn(move || {
        let reader = BufReader::new(stderr);
        let mut err = String::new();
        for line in reader.lines() {
            if let Ok(line) = line {
                eprintln!("{}", line.green());
                err.push_str(&line);
                err.push('\n');
            }
        }
        err
    });

    let stdout_output = stdout_thread.join().unwrap();
    let stderr_output = stderr_thread.join().unwrap();

    child.wait().expect("Command wasn't running");

    return format!("{}{}", stdout_output, stderr_output);
}

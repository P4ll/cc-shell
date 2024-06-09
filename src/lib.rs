#[allow(unused_imports)]
use std::fmt;
use std::fs;
#[allow(unused_imports)]
use std::io::{self, Write};

#[derive(Debug, Clone)]
pub enum Cmd<'a> {
    Exit,
    CmdNotFound(&'a str),
    Echo(&'a str),
    Type(&'a str),
}

impl<'a> fmt::Display for Cmd<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cmd::Exit => write!(f, "exit"),
            Cmd::Echo(_) => write!(f, "echo"),
            Cmd::Type(_) => write!(f, "type"),
            _ => unreachable!(),
        }
    }
}

pub fn parse_cmd(cmd: &str) -> Cmd {
    let lower_case_cmd = cmd.to_lowercase();
    let space_pos = cmd.find(' ');

    if let Some(pos) = space_pos {
        match &cmd[..pos] {
            "echo" => return Cmd::Echo(&cmd[pos + 1..]),
            "type" => return Cmd::Type(&cmd[pos + 1..]),
            "exit" => return Cmd::Exit,
            _ => {}
        }
    } else {
        match cmd {
            "echo" => return Cmd::Echo(""),
            "type" => return Cmd::Type(""),
            "exit" => return Cmd::Exit,
            _ => {}
        }
    }

    match lower_case_cmd.as_str() {
        "exit 0" => Cmd::Exit,
        _ => Cmd::CmdNotFound(cmd.trim()),
    }
}

pub fn exec_cmd(cmd: Cmd, env_paths: &[&str]) {
    match cmd {
        Cmd::CmdNotFound(inp) => println!("{}: command not found", inp.trim()),
        Cmd::Echo(inp) => println!("{}", inp),
        Cmd::Exit => unreachable!(),
        Cmd::Type(inp) => {
            exec_type_cmd(inp, env_paths);
        }
    }
}

fn exec_type_cmd(inp: &str, env_paths: &[&str]) {
    let inner_cmd = parse_cmd(inp);
    match inner_cmd {
        Cmd::CmdNotFound(cmd) => {
            let founded = get_in_path(env_paths, cmd);
            if let Some(prog_path) = founded {
                println!("{cmd} is {prog_path}")
            } else {
                println!("{cmd}: not found")
            }
        }
        _ => println!("{} is a shell builtin", inner_cmd),
    }
}

fn get_in_path<'a>(env_paths: &[&str], cmd: &'a str) -> Option<String> {
    // dbg!(env_paths);
    for env in env_paths {
        if let Ok(read_dir) = fs::read_dir(*env) {
            for entry in read_dir {
                // dbg!(&entry);
                if let Ok(file) = entry {
                    // dbg!(&file);
                    let tt = file.file_name();
                    if tt == cmd {
                        return file.path().to_str().map(str::to_string);
                    }
                    // dbg!(&tt);
                }
            }
        }
    }
    None
}
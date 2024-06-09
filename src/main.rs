#[allow(unused_imports)]
use std::fmt;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::env;
use anyhow::Result;

use shell_starter_rust::{exec_cmd, parse_cmd, Cmd};

fn main() -> Result<()> {
    let stdin = io::stdin();
    let mut input = String::new();

    let path_env_str = env::var("PATH")?;
    let path_envs: Vec<&str> = path_env_str.split(':').collect();
    // dbg!(&path_envs);

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        stdin.read_line(&mut input).unwrap();

        let cmd = parse_cmd(&input.trim());
        if let Cmd::Exit = cmd {
            break;
        }

        exec_cmd(cmd, &path_envs)?;
        input.clear();
    }
    Ok(())
}

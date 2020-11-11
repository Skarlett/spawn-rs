use std::process::{Command, Stdio};
use std::env;

fn help() {
    eprintln!(
        "{} <program> [<arg> ...]",
        env::args().nth(0).unwrap()
    )
}

fn main() {
    let show_pid = env::var("SPAWN_SHOW_PID").ok().is_some();

    let program = match env::args().nth(1) {
        Some(bin_name) => bin_name,
        None => { 
            help();
            return
        }
    };

    let pass_on_args: Vec<_> = env::args().skip(2).collect();

    let child = Command::new(program)
        .stdout(Stdio::null())
        .stdin(Stdio::inherit())
        .stderr(Stdio::null())
        .envs(env::vars())
        .args(&pass_on_args)
        .spawn()
        .expect("failed to execute slave");
    
    if show_pid {
        println!("[{}]", child.id())
    }
}

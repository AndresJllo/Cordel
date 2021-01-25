use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::process::{Command, Stdio};

fn into_stdio(stdio: (&str, &str)) -> (Stdio, Stdio) {
    let mut final_tup = (Stdio::null(), Stdio::null());
    
    final_tup.0 = match stdio.0 {
        "" => Stdio::inherit(),
        _ => Stdio::from(File::open(stdio.0).unwrap()),
    };
    final_tup.1 = match stdio.1 {
        "" => Stdio::inherit(),
        _ => Stdio::from(File::create(stdio.1).unwrap()),
    };

    final_tup
}

pub trait Target {
    fn exe_ord(&self, command: &str, args: Vec<&str>, stdio: (&str, &str)) {
        // let command = String::from(command);
        let (stdin, stdout) = into_stdio(stdio);

        println!("now running... {} {:?}", command, args);
        Command::new(command)
            .stdin(stdin)
            .stdout(stdout)
            .args(args)
            .status()
            .expect("can't run command");
    }

    fn compile(&self);
    fn run(&self);
    fn delet(&self);
    fn print_self(&self) -> String;
    fn get_infile(&self) -> String;
    fn get_outfile(&self) -> String;    
    fn main_loop(&mut self);
    
}

impl std::fmt::Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.print_self())
    }
}

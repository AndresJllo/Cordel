mod climsg;
mod filetarget;
mod kmp;
mod maketarget;
mod target;
mod tmaster;

use crate::climsg::CliMsg;

fn print_todo() {
    let msg = CliMsg::new(
        "TO-DO",
        vec![
            ">consider creating a menu that allows you",
            "to pick in what order makefiles get compiled",
            ">maybe make it so that all makefile outputs get",
            "run and deleted and not just the first one you find",
            "Rust is a language for niggers and trannies",
        ],
        "*",
        36,
        0,
        0,
    );
    println!("{}", msg);
}

fn main_loop() {
    println!("COmpile-Run-DElete-Loop(CORDEL) v.0.0.1");
    println!("type cordel -h and hit ENTER for help"); 
    let mut a = tmaster::TMaster::new();
    a.start();
    println!("bye");
}

fn main() {
    main_loop();
}

use crate::target::Target;
use crate::climsg::CliMsg; 
use std::fs;

pub struct MakeTarget {
    filename: String,
    target: Option<String>,
    remove: bool,
    input_file: String,
    output_file: String,
}

impl MakeTarget {
    pub fn new(filename: &str, remove: bool, input_file: &str, output_file: &str) -> MakeTarget {
        let filename = String::from(filename);
        let input_file = String::from(input_file);
        let output_file = String::from(output_file);
        MakeTarget {
            filename,
            target: None,
            remove,
            input_file,
            output_file, 
        }
    }

    pub fn find_target(&mut self) {
        let paths: Vec<String> = fs::read_dir("./")
            .unwrap()
            .map(|path| path.unwrap().file_name().into_string().unwrap())
            .collect();

        let msg = CliMsg::new(
            "WARNING",
            vec![
                "cordel will try to compile the makefile",
                "cordel is really dumb so please",
                "dont add anythin to the folder until",
                "compilation is done so it can find the output",
            ],
            "-",
            36,
            2,
            0,
        );

        println!("{}", msg); 
        self.compile();
        let paths2 = fs::read_dir("./")
            .unwrap()
            .map(|path| path.unwrap().file_name().into_string().unwrap());

        let target = paths2.filter(|file| !paths.contains(file)).next();

        let target = match target {
            Some(s) => s,
            _ => {
                println!("makefile didn't yield any output!");
                return;
            }
        };
        self.target = Some(target);
        if let Some(t) = &self.target {
            println!("{}", t);
        }
    }
}

impl Target for MakeTarget {
    fn compile(&self) {
        let args: Vec<&str> = vec!["-f", &self.filename];
        self.exe_ord("make", args, ("", ""));
    }

    fn run(&self) {
        let mut ordr = String::from("./");
        let mut args: Vec<&str> = Vec::new();
        match &self.target {
            Some(s) => {
                ordr += s;
                self.exe_ord(&ordr, args, (&self.input_file, &self.output_file));
            }
            _ => {
                println!("couldn't find makefile target!");
                return;
            }
        }
    }

    fn delet(&self) {
        let args: Vec<&str> = vec!["clean", "-f", &self.filename];
        self.exe_ord("make", args, ("", ""));
    }

    fn print_self(&self) -> String {
        String::from("filename: ") + &self.filename
    }

    fn get_infile(&self) -> String {
        self.input_file.clone()
    }
    fn get_outfile(&self) -> String {
        self.output_file.clone()
    }
    
    fn main_loop(&mut self) {
        self.find_target();
        self.run();
        if self.remove {
            self.delet();
        }
    }
}

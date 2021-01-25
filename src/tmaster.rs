use crate::climsg::CliMsg;
use crate::filetarget::FileTarget;
use crate::kmp::kmp;
use crate::maketarget::MakeTarget;
use crate::target::Target;

use std::env;
use std::fs;

use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::HashMap;

pub struct TMaster {
    orders: RefCell<Vec<Box<dyn Target>>>,
    makefiles: RefCell<Vec<String>>,
    args: Vec<String>,
    remove_all: bool,
    remove_mks: bool,
    do_mk: bool,
    flags: HashMap<String, bool>,
    ext: Vec<String>,
    help: bool,
}

impl TMaster {
    pub fn new() -> TMaster {
        let args: Vec<String> = env::args().collect();
        let makefiles = RefCell::new(Vec::new());
        let flags: HashMap<String, bool> = ["-rmk", "-ra", "-i", "-h"]
            .into_iter()
            .map(|f| (String::from(*f), false))
            .collect();

        let ext = [".cpp", ".c"]
            .into_iter()
            .map(|ext| String::from(*ext))
            .collect();

        TMaster {
            orders: RefCell::new(Vec::new()),
            makefiles,
            args,
            remove_all: false,
            remove_mks: false,
            do_mk: true,
            flags,
            ext,
            help: false,
        }
    }

    pub fn get_mk(&self) -> bool {
        self.do_mk
    }
}

impl TMaster {
    pub fn read_flags(&mut self) {
        let mut new_args = Vec::new();

        for arg in self.args.drain(..) {
            if let Some(x) = self.flags.get_mut(&arg) {
                *x = true;
            } else {
                new_args.push(arg)
            }
        }
        new_args.remove(0);
        self.args = new_args;
    }

    pub fn parse_flags(&mut self) {
        if self.flags["-ra"] {
            self.remove_all = true;
        }
        if self.flags["-rmk"] {
            self.remove_mks = true;
        }
        if self.flags["-i"] {
            self.do_mk = false;
        }
        if self.flags["-h"] {
            let msg = CliMsg::new(
                "HELP",
                vec![
                    "cordel compiles, runs, and optionally deletes",
                    "your c++ programs for you all in one go",
                    "",
                    "cordel aims to make your life 2% easier by reducing the",
                    "number of commands you have to run from 3 to 1.",
                    "it also does this automatically if it detects a makefile",
                    "in the folder.",
                    "",
                    "basic usage:",
                    "cordel [-ra|-rmk|-i|-h]* [FILENAME[.mk|.cpp] [-r] [<INPUTFILE] [>OUTPUTFILE]]*",
                    "",
                    "flags:",
                    "-ra removes all executable files compiled",
                    "-rmk removes all makefile outputs from makefiles",
                    "which weren't passed as arguments",
                    "-i ignores the makefiles that werent passed as arguments",
                    "-h prints this menu",
                    "-r removes the executable after its done",
                    "",
                    "warnings:",
                    ">cordel uses make to compile the files",
                    ">cordel relies on the standard names of things",
                    "in makefiles, specifically, it calls on a",
                    "target named \"clean\" as a way to remove makefile",
                    "output, so make sure a target under that name exists",
                    "in your makefile and contains the desired behaviour",
                    "",
                    "example:",
                    "in the same folder cordel came in, there should be a file",
                    "named nothing.cpp, try typing",
                    "cordel -r nothing.cpp",
                ],
                "?",
                36,
                2,
                0,
            );
            println!("{}", msg);
            self.help = true;
        }
    }

    pub fn find_makefile(&mut self) {
        self.makefiles = RefCell::new(
            fs::read_dir("./")
                .unwrap()
                .map(|path| path.unwrap().file_name().into_string().unwrap())
                .filter(|file| {
                    file == "makefile"
                        || file.ends_with(".mk")
                        || file == "GNUmakefile"
                        || file == "Makefile"
                })
                .collect(),
        );
    }

    pub fn add_makefile(&mut self) {
        let mut replace = Vec::new();
        let mut mk_ref = self.makefiles.borrow_mut();
        for mkfile in mk_ref.drain(..) {
            if !self.args.contains(&mkfile) {
                replace.push(mkfile);
            }
        }

        replace.into_iter().for_each(|x| {
            self.orders.borrow_mut().push(Box::new(MakeTarget::new(
                &x,
                self.remove_mks || self.remove_all,
                "",
                "",
            )))
        });
    }

    fn parse_ext(&self, filename: &str) -> String {
        let mut filename = String::from(filename);
        for e in self.ext.iter() {
            let mut index = kmp(e, &filename);
            while index < filename.len() {
                filename = String::from(&filename[..index]) + &filename[index + e.len()..];
                index = kmp(e, &filename);
            }
        }

        filename
    }

    pub fn add_ext(&mut self, ext: &str) {
        self.ext.push(String::from(ext));
        let temp_lambda = |a: &String, b: &String| -> Ordering {
            if a.len() > b.len() {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        };
        self.ext.sort_by(temp_lambda);
    }

    pub fn add_targets(&mut self) {
        let mut remove_flag = false;
        let mut input_file: &str = "";
        let mut output_file: &str = "";
        let mut in_target: &str = "";
        let mut t_index = 0;
        self.args.push(String::from("y.$EOL$.k")); // it will do the op in the loop
        while t_index < self.args.len() {
            let mut target = &self.args[t_index];
            if in_target == "" {
                in_target = target;
            } else if target == "-r" {
                remove_flag = true;
            } else if target == "-in" {
                input_file = &self.args[t_index + 1];
                t_index += 1;
            } else if target == "-out" {
                output_file = &self.args[t_index + 1];
                t_index += 1;
            } else {
                if in_target.ends_with(".mk") {
                    self.orders.borrow_mut().push(Box::new(MakeTarget::new(
                        in_target,
                        remove_flag || self.remove_all,
                        input_file,
                        output_file,
                    )));
                } else {
                    let in_target_temp = self.parse_ext(in_target);
                    self.orders.borrow_mut().push(Box::new(FileTarget::new(
                        &in_target_temp,
                        remove_flag || self.remove_all,
                        input_file,
                        output_file,
                    )));
                }
                remove_flag = false;
                input_file = "";
                output_file = "";
                in_target = target;
            }
            t_index += 1;
        }
    }

    pub fn execute_orders(&self) {
        for order in self.orders.borrow_mut().iter_mut() {
            let title = String::from("ORDER ") + &order.print_self();

            println!("{}", CliMsg::new_div(&title, "+-", 36, 2, 0));
            println!("input file: {} ", order.get_infile());
            println!("output file: {} ", order.get_outfile());                        
            order.main_loop();
            println!("{}", CliMsg::new_div("END OF ORDER", "+-", 36, 0, 2));
        }
    }

    pub fn start(&mut self) {
        self.read_flags();
        self.parse_flags();
        if !self.help {
            if self.get_mk() {
                self.find_makefile();
                self.add_makefile();
            }
            if self.args.len() > 0 {
                self.add_targets();
            }
            self.execute_orders();
        }
    }
}

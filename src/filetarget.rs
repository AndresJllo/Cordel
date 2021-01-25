use crate::target::Target;
pub struct FileTarget {
    target: String,
    remove: bool,
    input_file: String,
    output_file: String,
}

impl FileTarget {
    pub fn new(target: &str, remove: bool, input_file: &str, output_file: &str) -> FileTarget {
        let input_file = String::from(input_file);
        let output_file = String::from(output_file);
        FileTarget {
            target: String::from(target),
            remove,
            input_file,
            output_file,
        }
    }
}

impl Target for FileTarget {
    fn compile(&self) {
        let args: Vec<&str> = vec![&self.target];
        self.exe_ord("make", args, ("", ""));
    }

    fn run(&self) {
        let mut ordr = String::from("./");
        let mut args: Vec<&str> = Vec::new();
        ordr += &self.target;
        self.exe_ord(&ordr, args, (&self.input_file, &self.output_file));
    }

    fn delet(&self) {
        let args: Vec<&str> = vec![&self.target];
        self.exe_ord("rm", args, ("", ""));
    }
    fn print_self(&self) -> String {
        String::from("target: ") + &self.target
    }

    fn get_infile(&self) -> String {
        self.input_file.clone()
    }
    fn get_outfile(&self) -> String {
        self.output_file.clone()
    }

    fn main_loop(&mut self) {
        self.compile();
        self.run();
        if self.remove {
            self.delet();
        }
    }
}

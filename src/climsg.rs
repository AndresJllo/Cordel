use std::iter;

fn gen_pad(num: usize) -> String {
    let mut pad = String::new();
    for _ in 0..num {
        pad += "\n";
    }
    pad
}

pub struct CliMsg<'a> {
    title: &'a str,
    lines: Vec<&'a str>,
    divider: String,
    repr: String,
    padding: usize,
    i_padding: usize,
}

impl<'a> std::fmt::Display for CliMsg<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.repr)
    }
}

impl<'a> CliMsg<'a> {

    /*
    Look billy, its bad software engineering
    maybe add a div struct in the future retard
    make them share a common interface and just
    create a whole cli library while youre at it 
    why dont ya
     */
    pub fn new_div(title: &'a str, sep: &str, sep_len: usize, padding: usize, i_padding: usize) -> CliMsg<'a> {
        let mut repr = String::new();
        let pad = gen_pad(padding);
        repr+=&pad; 
        repr += title;

        let num_s: usize = (sep_len - title.len())/sep.len();
        let divider: String = iter::repeat(String::from(sep))
            .take(num_s)
            .collect();
        repr += &divider;
        let ipad = gen_pad(i_padding);
        repr += &ipad;
        
        CliMsg {
            title,
            lines: Vec::new(),
            divider,
            repr,
            padding,
            i_padding,            
        }
    }

    pub fn new(title: &'a str, lines: Vec<&'a str>, sep: &str, sep_len: usize, padding: usize, i_padding: usize) -> CliMsg<'a> {
        // let title = String::from(title);
        // let lines = lines.into_iter().map(|line| String::from(line)).collect();
        let divider: String = iter::repeat(String::from(sep))
            .take(sep_len - title.len())
            .collect();
        let mut a = CliMsg {
            title,
            lines,
            divider,
            repr: String::new(),
            padding,
            i_padding,
        };
        a.gen_repr();
        a
    }

}

impl<'a> CliMsg<'a> {
    fn gen_repr(&mut self) {
        let mut repr = String::new();
        let pad = gen_pad(self.padding);
        repr += &pad;
        repr += self.title;
        repr += &self.divider;
        repr += "\n";
        let ipad = gen_pad(self.i_padding);
        repr += &ipad;

        for line in self.lines.drain(..) {
            repr += line;
            repr += "\n";
        }

        repr += &ipad;
        repr += &self.divider;
        let comp: String = iter::repeat(self.divider[0..1].to_string())
            .take(self.title.len())
            .collect();
        repr += &comp;
        repr += &pad;
        self.repr = repr;
    }
}

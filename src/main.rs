extern crate getopts;
extern crate regex;

use std::os;


fn main() {
    let args = os::args();
    let program = args[0].clone();
    let opt_matches = match getopts::getopts(args.tail(), options().as_slice()) {
        Ok(m) => { m }
        Err(f) => {
            println!("{}\n", f.to_string());
            print_usage(program.as_slice());
            return;
        }
    };
    if opt_matches.opt_present("h") {
        print_usage(program.as_slice());
        return;
    }
}

struct CmdContext {
    cmd_args:   Vec<String>,
    in_delim:   Vec<regex::Regex>,
    out_delim:  Vec<regex::Regex>,
    nprocs:     u32,
    empty:      String,
}

fn shape(args: Vec<String>) {
}

fn reshape(args: Vec<String>) {
}

fn map(args: Vec<String>) {
}

fn agg(args: Vec<String>) {
}


fn options() -> Vec<getopts::OptGroup> {
    vec![
        getopts::optflag("h", "help", "Print this help message"),
        getopts::optopt("d", "delim", "Specify axis delimiters. [Default:\\n/\\s]", "DELIM"),
        getopts::optopt("l", "odelim", "Specify axis delimiters. [Default:\\n/\\s]", "ODELIM"),
        getopts::optopt("e", "empty", "Specify the string that should be used to represent empty cells", "EMPTY"),
        getopts::optopt("", "nprocs", "Maximum number of subprocesses or threads that can be spawned. [Default:16]", "NPROCS"),
    ]
}

fn print_usage(program: &str) { 
    let opts = options();
    let commands = "Commands:
    shape                   Get the shape of this matrix
    reshape <new_shape>     Reshape this matrix into a new one
    map <cmd>               Map a command on each selected cell
    agg <axes> <cmd>        Aggregate a matrix along the specified axes
                            using a command.";
    let brief = format!("USAGE:\n    {} [options] <selector> [<cmd>] [<input_file>...]\n\n{}", program, commands);
    print!("{}", getopts::usage(brief.as_slice(), opts.as_slice()));
}

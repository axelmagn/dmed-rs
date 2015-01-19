extern crate getopts;

use std::os;

fn main() {
    let args = os::args();
    let program = args[0].clone();
    print_usage(program.as_slice());
}


fn options() -> Vec<getopts::OptGroup> {
    vec![
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
    let brief = format!("USAGE:\n{} [options] <selector> [<cmd>] [<input_file>...]\n\n{}", program, commands);
    print!("{}", getopts::usage(brief.as_slice(), opts.as_slice()));
}

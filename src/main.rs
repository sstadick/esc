use std::{
    error::Error,
    io::{BufReader, BufWriter, Read, Write},
};

use snailquote::{escape, unescape};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt()]
struct SharedOpts {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt)]
enum Command {
    Escape,
    Unescape,
}

fn main() -> Result<(), Box<dyn Error>> {
    let shared_opts = SharedOpts::from_args();

    let mut input_string = String::new();
    let input = std::io::stdin();
    BufReader::new(input.lock()).read_to_string(&mut input_string)?;

    let output = std::io::stdout();
    let mut output = BufWriter::new(output.lock());

    let out_value = match shared_opts.cmd {
        Command::Escape => escape(&input_string),
        Command::Unescape => unescape(&input_string)?.into(),
    };

    output.write_all(out_value.as_bytes())?;

    Ok(())
}

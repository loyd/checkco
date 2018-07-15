extern crate checkco;
#[macro_use]
extern crate structopt;
extern crate serde_json;

use std::error::Error;
use std::fs::File;
use std::path::PathBuf;

use checkco::Schema;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct CLI {
    #[structopt(parse(from_os_str))]
    derived: PathBuf,
    #[structopt(parse(from_os_str))]
    base: PathBuf,
}

fn main() -> Result<(), Box<Error>> {
    let options = CLI::from_args();

    let derived: Schema = serde_json::from_reader(File::open(options.derived)?)?;
    let base: Schema = serde_json::from_reader(File::open(options.base)?)?;

    let result = checkco::check(derived, base);

    println!(">> {:#?}", result);

    Ok(())
}

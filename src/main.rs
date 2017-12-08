
extern crate clap;
extern crate flate2;

pub mod bytecode;

use clap::*;

use std::fs::File;
use std::io::Read;
use std::path::*;

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

fn read_rlib(filename: &Path) -> Result<Vec<u8>> {
    let mut f = File::open(filename)?;
    let mut buf = Vec::new();
    f.read_to_end(&mut buf)?;
    Ok(buf)
}

fn main() {
    let matches = App::new("cargo-nvbuild")
        .bin_name("cargo")
        .subcommand(SubCommand::with_name("nvbuild").arg(
            Arg::with_name("input").help("Input *.rlib file").required(
                true,
            ),
        ))
        .get_matches();
    let matches = matches.subcommand_matches("nvbuild").unwrap();

    let input = matches.value_of("input").unwrap();
    let bin = read_rlib(Path::new(input)).unwrap();
    let bc = bytecode::DecodedBytecode::new(&bin).unwrap();
    println!("Identifier = {:?}", bc.identifier());
    println!("ByteCode   = {:?}", bc.bytecode());
}

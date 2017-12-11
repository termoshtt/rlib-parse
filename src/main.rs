
extern crate clap;
extern crate flate2;

pub mod bytecode;

use clap::*;

use std::fs::File;
use std::io::{Read, Write};
use std::path::*;

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

fn read_rlib(filename: &Path) -> Result<Vec<u8>> {
    let mut f = File::open(filename)?;
    let mut buf = Vec::new();
    f.read_to_end(&mut buf)?;
    Ok(buf)
}

fn write_bc(filename: &Path, data: &[u8]) -> Result<()> {
    let mut f = File::create(filename)?;
    f.write_all(data)?;
    Ok(())
}

fn main() {
    let matches = App::new("nvbuild")
        .arg(Arg::with_name("input").help("Input *.rlib file").required(
            true,
        ))
        .get_matches();
    let input = matches.value_of("input").unwrap();
    let bin = read_rlib(Path::new(input)).unwrap();
    let decoded = bytecode::DecodedBytecode::new(&bin).unwrap();
    let name = format!("{}.bc", decoded.identifier());
    let bc = decoded.bytecode();
    write_bc(&Path::new(&name), &bc).unwrap();
}

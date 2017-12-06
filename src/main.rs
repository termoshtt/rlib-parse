#![feature(rustc_private)]

extern crate rustc_trans;
extern crate clap;

use rustc_trans::back::bytecode;
use clap::{App, Arg};

use std::fs::File;
use std::io::Read;
use std::path::*;

fn read_rlib(filename: &Path) -> Result<Vec<u8>, Box<::std::error::Error>> {
    let mut f = File::open(filename)?;
    let mut buf = Vec::new();
    f.read_to_end(&mut buf)?;
    Ok(buf)
}

fn main() {
    let app =
        App::new("cargo-nvbuild").arg(Arg::with_name("input").help("Input *.rlib file").required(
            true,
        ));
    let matches = app.get_matches();

    let input = matches.value_of("input").unwrap();
    let bin = read_rlib(Path::new(input)).unwrap();
    let bc = bytecode::DecodedBytecode::new(&bin).unwrap();
    println!("Identifier = {:?}", bc.identifier());
    println!("ByteCode   = {:?}", bc.bytecode());
}

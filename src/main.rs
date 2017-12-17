
extern crate clap;
extern crate flate2;
extern crate glob;

pub mod bytecode;

use clap::*;
use glob::glob;

use std::fs::File;
use std::io::{Read, Write};
use std::path::*;
use std::process::Command;

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

fn read_binary(filename: &Path) -> Result<Vec<u8>> {
    let mut f = File::open(filename)?;
    let mut buf = Vec::new();
    f.read_to_end(&mut buf)?;
    Ok(buf)
}

fn write_binary(filename: &Path, data: &[u8]) -> Result<()> {
    let mut f = File::create(filename)?;
    f.write_all(data)?;
    Ok(())
}

fn parse_path(path: &Path) -> (&Path, &str) {
    let filename = path.file_name().unwrap().to_str().unwrap();
    let dir = path.parent().unwrap();
    (dir, filename)
}

fn main() {
    let matches = App::new("rlib-parse")
        .arg(Arg::with_name("input").help("Input *.rlib file").required(
            true,
        ))
        .get_matches();

    let rlib = {
        let input = matches.value_of("input").unwrap();
        if !input.ends_with(".rlib") {
            panic!("Not a rlib: {}", input);
        }
        Path::new(input)
    };
    let (dir, fname) = parse_path(&rlib);

    Command::new("ar")
        .args(&["x", fname])
        .current_dir(dir)
        .status()
        .unwrap();

    let pat_rustobj = format!("{}/*.bytecode.encoded", dir.display());
    for rust_obj in glob(&pat_rustobj).unwrap() {
        let bin = read_binary(&rust_obj.unwrap()).unwrap();
        let decoded = bytecode::DecodedBytecode::new(&bin).unwrap();
        let bc = decoded.bytecode();
        let bc_file = dir.join(format!("{}.bc", decoded.identifier()));
        write_binary(&bc_file, &bc).unwrap();
    }
}

use std::{
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
};

use crate::vmf2::{generic::BasicParser, vmf::Vmf};

mod vmf2;

fn main() -> std::io::Result<()> {
    let mut two_cube = String::new();
    BufReader::new(File::open("mp_coop_doors.vmf")?).read_to_string(&mut two_cube)?;

    let mut p: BasicParser = BasicParser { input: &two_cube };

    let root = p.read_tree().unwrap();
    let root = Vmf::parse(root);

    println!("{}", root.as_generic().to_string());

    println!("8888888888888888888888");

    println!("{root:#?}");

    BufWriter::new(File::create("coop_TEST.vmf")?)
        .write_all(root.as_generic().to_string().as_bytes())?;

    Ok(())
}
use std::{
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
};

use bevy::prelude::*;

use crate::vmf2::{vmf::Vmf, generic::GenericNode};

mod vmf2;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(hello_world)
        .run();
}

fn hello_world() {
    println!("Hello world!");
}


fn old_main() -> std::io::Result<()> {
    let mut two_cube = String::new();
    BufReader::new(File::open("testing/mp_coop_doors.vmf")?).read_to_string(&mut two_cube)?;

    let root = GenericNode::parse(&two_cube).unwrap();

    let root: Vmf = Vmf::parse(root);

    println!("{}", root.as_generic().to_string());

    println!("8888888888888888888888");

    println!("{root:#?}");

    BufWriter::new(File::create("testing/coop_TEST.vmf")?)
        .write_all(root.as_generic().to_string().as_bytes())?;

    Ok(())
}
use std::{fs::File, io::{BufReader, Read, BufWriter, Write}};

use bevy::{prelude::{Resource, FromWorld, World, Handle}, asset::Asset, reflect::TypeUuid};

use super::{vmf::Vmf, generic::GenericNode};

#[derive(Debug,Resource,Default)]
pub struct ActiveVmf {
    pub active: Option<Handle<VmfFile>>
}

#[derive(Debug,TypeUuid)]
#[uuid="9497d134-0aee-4af7-9ae0-a5c5268eeb8e"]
pub struct VmfFile {
    file: File,
    pub vmf: Vmf
}

impl VmfFile {
    pub fn open(name: &str) -> VmfFile {
        let file = File::open(name).unwrap();

        let mut s = String::new();
        BufReader::new(&file).read_to_string(&mut s).unwrap();

        let generic = GenericNode::parse(&s).unwrap();

        let vmf = Vmf::parse(generic);

        Self {
            file,
            vmf
        }
    }

    pub fn save(&self, name: &str) {
        let file = File::create(name).unwrap();

        BufWriter::new(&file).write_all(self.vmf.as_generic().to_string().as_bytes()).unwrap();
    }
}


use std::path::PathBuf;

use bevy::{
    prelude::{Handle, Resource},
    reflect::TypeUuid,
};

use super::{generic::GenericNode, vmf::Vmf};

#[derive(Debug, Resource, Default)]
pub struct ActiveVmf {
    pub active: Option<Handle<VmfFile>>,
}

#[derive(Debug, TypeUuid)]
#[uuid = "9497d134-0aee-4af7-9ae0-a5c5268eeb8e"]
pub struct VmfFile {
    pub path: PathBuf,
    pub vmf: Vmf,
}

impl VmfFile {
    pub fn open(path: PathBuf) -> VmfFile {
        let in_file = std::fs::read_to_string(&path).unwrap();

        let generic = GenericNode::parse(&in_file).unwrap();

        let vmf = Vmf::parse(generic);

        Self { path, vmf }
    }

    pub fn save(&self) {
        // println!("{:?}",self.path);
        std::fs::write(&self.path, self.vmf.as_generic().to_string().as_bytes()).unwrap();
    }

    pub fn save_as(&mut self, path: PathBuf) {
        self.path = path;
        self.save();
    }
}

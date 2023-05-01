use std::{
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
};

// use serde::{Deserialize, Serialize};

use crate::vmf2::{generic::BasicParser, vmf::Vmf};

// #[derive(Debug, Serialize, Deserialize)]
// #[serde(rename = "solid")]
// struct TestSolid {
//     sides: Vec<TestSide>,
// }

// #[derive(Debug, Serialize, Deserialize)]
// #[serde(rename = "side")]
// struct TestSide {
//     height: u32,
// }

// mod vmf;
mod vmf2;

// use crate::vmf::vmf::{VersionInfo, VisGroups, Vmf};

fn main() -> std::io::Result<()> {
    let mut two_cube = String::new();
    BufReader::new(File::open("mp_coop_doors.vmf")?).read_to_string(&mut two_cube)?;

    let mut p = BasicParser { input: &two_cube };

    let root = p.read_tree().unwrap();
    let root = Vmf::parse(root);

    println!("{}", root.as_generic().to_string());

    println!("8888888888888888888888");

    println!("{root:#?}");

    BufWriter::new(File::create("coop_TEST.vmf")?)
        .write_all(root.as_generic().to_string().as_bytes())?;

    Ok(())
}

/*
fn main() -> std::io::Result<()> {
    // let vis = VisGroups { groups: Vec::new() };
    // let vis_string = &vmf::ser::ser::to_string(&vis).unwrap();
    // println!("{vis_string}");
    // let vis2: VisGroups = vmf::de::de::from_str(&vis_string).unwrap();
    // println!("{vis:?} =?= {vis2:?}");

    // let ver = VersionInfo {
    //     editor_version: 400,
    //     editor_build: 8997,
    //     map_version: 1,
    //     format_version: 100,
    //     prefab: 0,
    // };

    // let ver_string = vmf::ser::ser::to_string(&ver).unwrap();
    // println!("{ver_string}");
    // let ver2: VersionInfo = vmf::de::de::from_str(&ver_string).unwrap();
    // println!("{ver:?} =?= {ver2:?}");
    // println!("{ver:?}");

    // let solid = TestSolid {
    //     sides: vec![
    //         TestSide { height: 1 },
    //         TestSide { height: 2 },
    //         TestSide { height: 10 },
    //         TestSide { height: 4 },
    //     ],
    // };

    // let solid_string = vmf::ser::ser::to_string(&solid).unwrap();
    // println!("{solid_string}");
    // let solid2: TestSolid = vmf::de::de::from_str(&solid_string).unwrap();

    // println!("{solid:#?}\n=?=\n{solid2:#?}");
    // println!("{solid:#?}");

    let mut two_cube = String::new();
    BufReader::new(File::open("mp_coop_doors.vmf")?).read_to_string(&mut two_cube)?;

    let temp = vmf::de::de::from_str::<Vmf>(&two_cube);

    /*
    hidden
    solid
    visgroupid
    groupid
    editor

     */

    println!("{temp:#?}");

    let Vmf(
        version_info,
        vis_groups,
        view_settings,
        world,
        entities,
        hidden_entities,
        cameras,
        cordons,
    ) = temp.unwrap();

    // let vmf: Vmf = vmf::de::de::from_str(&two_cube).unwrap();

    // BufWriter::new(File::create("2_cube_TEST.vmf")?)
    //     .write_all(&vmf::ser::ser::to_string(&vmf).unwrap().as_bytes())?;

    /*
    Main TODO items: fully parse in everything like planes and uvs, right now I
    am just using strings but I think I could make them better, I just need
    to do a custom implementation of the what the Serialize/Deserialize macro normally does?

    Then I need to get entities in there working

    Stress test with better maps

    Just search for todo in the code base
    sleep now
     */

    Ok(())
}

*/

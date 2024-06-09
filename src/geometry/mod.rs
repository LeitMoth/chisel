use crate::vmf2::vmf;
use bevy::{
    prelude::*,
    render::{mesh::Indices, render_asset::RenderAssetUsages, render_resource::PrimitiveTopology},
};

#[derive(Debug)]
pub struct StandardPlane {
    pub normal: Vec3,
    pub d: f32,
}

impl StandardPlane {
    pub fn new(vmf_plane: &vmf::Plane) -> Self {
        // https://tutorial.math.lamar.edu/classes/calcIII/EqnsOfPlanes.aspx

        let p = vmf_plane.points[0].new_vec3();
        let q = vmf_plane.points[1].new_vec3();
        let r = vmf_plane.points[2].new_vec3();

        let p_to_q = q - p;
        let p_to_r = r - p;

        let c = p_to_q.cross(p_to_r);

        let w = p.dot(c);

        if w.abs() > 20_000_000.0 {
            dbg!(&(p, q, r, c, w));
        }

        StandardPlane { normal: c, d: w }
    }

    pub fn intersection_point(&self, p1: &Self, p2: &Self) -> Option<Vec3> {
        let s = Mat3 {
            x_axis: self.normal,
            y_axis: p1.normal,
            z_axis: p2.normal,
        }
        .transpose();

        let d_col = Vec3::new(self.d, p1.d, p2.d);

        if s.determinant() == 0.0 {
            None
        } else {
            let x = s.inverse() * d_col;
            // if x.length() > 20_000.0 {
            //     None
            // } else {
            //     Some(x)
            // }
            Some(x)
        }
    }
}

pub fn planes_to_sides(planes: &[StandardPlane]) -> Vec<Vec<Vec3>> {
    let mut sides: Vec<Vec<Vec3>> = Vec::new();

    for (i, p1) in planes.iter().enumerate() {
        let mut points = Vec::new();

        let mut start = None;

        'outer: for (j, p2) in planes.iter().enumerate() {
            for (k, p3) in planes.iter().enumerate() {
                if i != j && j != k && i != k {
                    if let Some(point) = p1.intersection_point(p2, p3) {
                        points.push(point);
                        start = Some((j, k));
                        break 'outer;
                    }
                }
            }
        }

        if start.is_none() {
            break;
        }

        let (start_j, start_k) = start.unwrap();
        let mut j = start_j;
        let mut k = start_k;

        fn find_with_with_without(
            planes: &[StandardPlane],
            required: usize,
            with: usize,
            without: usize,
        ) -> Option<(usize, Vec3)> {
            for (i, plane) in planes.iter().enumerate() {
                if i != required && i != with && i != without {
                    if let Some(point) = plane.intersection_point(&planes[required], &planes[with])
                    {
                        return Some((i, point));
                    }
                }
            }
            None
        }

        while let Some((index, point)) = find_with_with_without(planes, i, j, k) {
            // println!("estoy loopin? {i} {j} {k}");
            points.push(point);
            k = j;
            j = index;

            if j == start_j && k == start_k || j == start_k && k == start_j {
                break;
            }
        }

        sides.push(points);
    }

    sides
}

pub fn side_to_triangles(side: Vec<Vec3>) -> Mesh {
    let len = side.len();
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, side);
    let mut idx = Vec::new();
    for x in 2..len - 1 {
        idx.push((x + 1) as u16);
        idx.push(x as u16);
        idx.push(1);
    }
    mesh.insert_indices(Indices::U16(idx));
    mesh.duplicate_vertices();
    mesh.compute_flat_normals();

    mesh
}

pub fn side_to_lines(side: Vec<Vec3>) -> Mesh {
    let mut linemesh = Mesh::new(
        PrimitiveTopology::LineStrip,
        RenderAssetUsages::RENDER_WORLD,
    );
    linemesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, side);
    // linemesh.insert_indices(None);

    linemesh
}

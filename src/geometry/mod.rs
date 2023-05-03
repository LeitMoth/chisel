use crate::vmf2::vmf;
use bevy::{
    prelude::{*},
    render::{mesh::Indices, render_resource::PrimitiveTopology},
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

        StandardPlane { normal: c, d: w }
    }

    pub fn intersection_point(&self, p1: &Self, p2: &Self) -> Option<Vec3> {
        // https://en.wikipedia.org/wiki/Cramer%27s_rule

        // Keep in mind these arrays are columns. This would have been much cleaner if I could make one out of rows...
        let s = Mat3::from_cols_array_2d(&[
            [self.normal.x, p1.normal.x, p2.normal.x],
            [self.normal.y, p1.normal.y, p2.normal.y],
            [self.normal.z, p1.normal.z, p2.normal.z],
        ]);

        let d_col = Vec3::new(self.d, p1.d, p2.d);

        let mut x_top = s;
        *x_top.col_mut(0) = d_col;

        let mut y_top = s;
        *y_top.col_mut(1) = d_col;

        let mut z_top = s;
        *z_top.col_mut(2) = d_col;

        let s_det = s.determinant();

        if s_det == 0.0 {
            // println!("{self:?} <> {p1:?} <> {p2:?}");

            // println!("Det was 0!!!");
            return None;
        }

        let x = x_top.determinant() / s_det;
        let y = y_top.determinant() / s_det;
        let z = z_top.determinant() / s_det;

        Some(Vec3::new(x, y, z))
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
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, side);
    let mut idx = Vec::new();
    for x in 2..len - 1 {
        idx.push((x + 1) as u16);
        idx.push(x as u16);
        idx.push(1);
    }
    mesh.set_indices(Some(Indices::U16(idx)));
    mesh.duplicate_vertices();
    mesh.compute_flat_normals();

    mesh
}

pub fn side_to_lines(side: Vec<Vec3>) -> Mesh {
    let mut linemesh = Mesh::new(PrimitiveTopology::LineStrip);
    linemesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, side);
    linemesh.set_indices(None);

    linemesh
}

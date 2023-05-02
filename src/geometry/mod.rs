use bevy::prelude::{*, shape::Plane};
use crate::vmf2::vmf;


#[derive(Debug)]
pub struct StandardPlane {
    pub normal: Vec3,
    pub d: f32
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

        StandardPlane {
            normal: c,
            d: w,
        }
    }

    pub fn intersection_point(&self, p1: &Self, p2: &Self) -> Option<Vec3> {
        // https://en.wikipedia.org/wiki/Cramer%27s_rule

        // Keep in mind these arrays are columns. This would have been much cleaner if I could make one out of rows...
        let s = Mat3::from_cols_array_2d(&[
            [self.normal.x, p1.normal.x, p2.normal.x],
            [self.normal.y, p1.normal.y, p2.normal.y],
            [self.normal.z, p1.normal.z, p2.normal.z],
        ]);

        let d_col = Vec3::new(self.d,p1.d,p2.d);

        let mut x_top = s.clone();
        *x_top.col_mut(0) = d_col;

        let mut y_top = s.clone();
        *y_top.col_mut(1) = d_col;

        let mut z_top = s.clone();
        *z_top.col_mut(2) = d_col;

        let s_det = s.determinant();

        if s_det == 0.0 {
            println!("{self:?} <> {p1:?} <> {p2:?}");

            println!("Det was 0!!!");
            return None;
        }

        let x = x_top.determinant() / s_det;
        let y = y_top.determinant() / s_det;
        let z = z_top.determinant() / s_det;

        Some(Vec3::new(x,y,z))
    }
}

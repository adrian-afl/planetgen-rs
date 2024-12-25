use glam::{DMat4, DVec2, DVec3, DVec4, Mat4};
use std::f64::consts::PI;

pub struct CubemapDataLayer<const RES: usize> {
    px: [f64; RES],
    py: [f64; RES],
    pz: [f64; RES],

    nx: [f64; RES],
    ny: [f64; RES],
    nz: [f64; RES],
}

enum CubeMapFace {
    PX,
    PY,
    PZ,
    NX,
    NY,
    NZ,
}

fn create_projection(face: &CubeMapFace) -> DMat4 {
    match (face) {
        CubeMapFace::PX => {
            DMat4::perspective_rh_gl(PI / 2.0, 1.0, 0.01, 10.0).mul_mat4(&DMat4::look_to_rh(
                DVec3::new(0.0, 0.0, 0.0),
                DVec3::new(1.0, 0.0, 0.0),
                DVec3::new(0.0, 1.0, 0.0),
            ))
        }
        CubeMapFace::PY => {
            DMat4::perspective_rh_gl(PI / 2.0, 1.0, 0.01, 10.0).mul_mat4(&DMat4::look_to_rh(
                DVec3::new(0.0, 0.0, 0.0),
                DVec3::new(0.0, 1.0, 0.0),
                DVec3::new(0.0, 0.0, -1.0),
            ))
        }
        CubeMapFace::PZ => {
            DMat4::perspective_rh_gl(PI / 2.0, 1.0, 0.01, 10.0).mul_mat4(&DMat4::look_to_rh(
                DVec3::new(0.0, 0.0, 0.0),
                DVec3::new(0.0, 0.0, 1.0),
                DVec3::new(0.0, 1.0, 0.0),
            ))
        }
        CubeMapFace::NX => {
            DMat4::perspective_rh_gl(PI / 2.0, 1.0, 0.01, 10.0).mul_mat4(&DMat4::look_to_rh(
                DVec3::new(0.0, 0.0, 0.0),
                DVec3::new(-1.0, 0.0, 0.0),
                DVec3::new(0.0, 1.0, 0.0),
            ))
        }
        CubeMapFace::NY => {
            DMat4::perspective_rh_gl(PI / 2.0, 1.0, 0.01, 10.0).mul_mat4(&DMat4::look_to_rh(
                DVec3::new(0.0, 0.0, 0.0),
                DVec3::new(0.0, -1.0, 0.0),
                DVec3::new(0.0, 0.0, 1.0),
            ))
        }
        CubeMapFace::NZ => {
            DMat4::perspective_rh_gl(PI / 2.0, 1.0, 0.01, 10.0).mul_mat4(&DMat4::look_to_rh(
                DVec3::new(0.0, 0.0, 0.0),
                DVec3::new(0.0, 0.0, -1.0),
                DVec3::new(0.0, 1.0, 0.0),
            ))
        }
    }
}

fn project_direction(face: &CubeMapFace, coord: DVec3) -> Option<DVec2> {
    let projection = create_projection(face);
    let transformed = projection.mul_vec4(DVec4::from((coord, 0.0)));
    if (transformed.x < -1.0
        || transformed.x > 1.0
        || transformed.y < -1.0
        || transformed.y > 1.0
        || transformed.z < -0.0)
    {
        return None;
    }
    return Some(DVec2::new(transformed.x, transformed.y) * 0.5 + 0.5);
}

fn get_face(coord: DVec3) -> CubeMapFace {
    let projected = project_direction(&CubeMapFace::PX, coord);
    if projected.is_some() {
        return CubeMapFace::PX;
    }
    let projected = project_direction(&CubeMapFace::PY, coord);
    if projected.is_some() {
        return CubeMapFace::PY;
    }
    let projected = project_direction(&CubeMapFace::PZ, coord);
    if projected.is_some() {
        return CubeMapFace::PZ;
    }
    let projected = project_direction(&CubeMapFace::NX, coord);
    if projected.is_some() {
        return CubeMapFace::NX;
    }
    let projected = project_direction(&CubeMapFace::NY, coord);
    if projected.is_some() {
        return CubeMapFace::NY;
    }
    let projected = project_direction(&CubeMapFace::NZ, coord);
    // if projected.is_some() {
    //     return CubeMapFace::NZ;
    // }
    CubeMapFace::NZ // nothing else left
}

impl<const RES: usize> CubemapDataLayer<RES> {
    fn set(&mut self, coord: DVec3, value: f64) {
        let face = get_face(coord);
        let uv01 = project_direction(&face, coord).unwrap();
        let uv = (uv01 * (RES as f64)).floor();
        let index = (uv.y * (RES as f64) + uv.x) as usize;
        match face {
            CubeMapFace::PX => self.px[index] = value,
            CubeMapFace::PY => self.py[index] = value,
            CubeMapFace::PZ => self.pz[index] = value,
            CubeMapFace::NX => self.nx[index] = value,
            CubeMapFace::NY => self.nx[index] = value,
            CubeMapFace::NZ => self.nx[index] = value,
        }
    }

    fn get(&mut self, coord: DVec3) -> f64 {
        let face = get_face(coord);
        let uv01 = project_direction(&face, coord).unwrap();
        let uv = (uv01 * (RES as f64)).floor();
        let index = (uv.y * (RES as f64) + uv.x) as usize;
        match face {
            CubeMapFace::PX => self.px[index],
            CubeMapFace::PY => self.py[index],
            CubeMapFace::PZ => self.pz[index],
            CubeMapFace::NX => self.nx[index],
            CubeMapFace::NY => self.nx[index],
            CubeMapFace::NZ => self.nx[index],
        }
    }
}

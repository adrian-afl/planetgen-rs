use crate::math_util::mix;
use glam::{DMat4, DVec2, DVec3, DVec4, Mat4, Vec4Swizzles};
use std::f64::consts::PI;
use std::fmt;

pub struct CubeMapDataLayer<const RES: usize> {
    px: Vec<f64>,
    py: Vec<f64>,
    pz: Vec<f64>,

    nx: Vec<f64>,
    ny: Vec<f64>,
    nz: Vec<f64>,
}

pub enum CubeMapFace {
    PX,
    PY,
    PZ,
    NX,
    NY,
    NZ,
}

impl fmt::Display for CubeMapFace {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CubeMapFace::PX => write!(f, "{}", "PX"),
            CubeMapFace::PY => write!(f, "{}", "PY"),
            CubeMapFace::PZ => write!(f, "{}", "PZ"),
            CubeMapFace::NX => write!(f, "{}", "NX"),
            CubeMapFace::NY => write!(f, "{}", "NY"),
            CubeMapFace::NZ => write!(f, "{}", "NZ"),
        }
    }
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
    Some(DVec2::new(transformed.x, transformed.y) * 0.5 + 0.5)
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
    if projected.is_some() {
        return CubeMapFace::NZ;
    }
    panic!("Impossible situation - no face found")
}

impl<const RES: usize> CubeMapDataLayer<RES> {
    pub fn new() -> CubeMapDataLayer<RES> {
        CubeMapDataLayer {
            px: vec![0.0; RES * RES],
            py: vec![0.0; RES * RES],
            pz: vec![0.0; RES * RES],

            nx: vec![0.0; RES * RES],
            ny: vec![0.0; RES * RES],
            nz: vec![0.0; RES * RES],
        }
    }

    fn is_out_of_bounds(x: isize, y: isize) -> bool {
        (x as usize) >= RES || (y as usize) >= RES || x < 0 || y < 0
    }

    pub fn set_pixel(&mut self, face: &CubeMapFace, x: usize, y: usize, value: f64) {
        let index = y * (RES) + x;
        match face {
            CubeMapFace::PX => self.px[index] = value,
            CubeMapFace::PY => self.py[index] = value,
            CubeMapFace::PZ => self.pz[index] = value,
            CubeMapFace::NX => self.nx[index] = value,
            CubeMapFace::NY => self.ny[index] = value,
            CubeMapFace::NZ => self.nz[index] = value,
        }
    }

    pub fn pixel_coords_to_direction(&self, face: &CubeMapFace, x: usize, y: usize) -> DVec3 {
        let inv_projection = create_projection(face).inverse();
        let uvx = x as f64 / RES as f64;
        let uvy = y as f64 / RES as f64;
        let clip = DVec4::new(uvx * 2.0 - 1.0, uvy * 2.0 - 1.0, 0.1, 1.0);
        let transformed = inv_projection * clip;
        transformed.xyz() / transformed.w
    }

    // TODO if this is to be used, it needs to also do bilinear filtering
    // fn set(&mut self, coord: DVec3, value: f64) {
    //     let face = get_face(coord);
    //     let uv01 = project_direction(&face, coord).unwrap();
    //     let uv = (uv01 * (RES as f64)).floor();
    //     let index = (uv.y * (RES as f64) + uv.x) as usize;
    //     match face {
    //         CubeMapFace::PX => self.px[index] = value,
    //         CubeMapFace::PY => self.py[index] = value,
    //         CubeMapFace::PZ => self.pz[index] = value,
    //         CubeMapFace::NX => self.nx[index] = value,
    //         CubeMapFace::NY => self.nx[index] = value,
    //         CubeMapFace::NZ => self.nx[index] = value,
    //     }
    // }

    pub fn get_pixel(&self, face: &CubeMapFace, x: usize, y: usize) -> f64 {
        let index = y * (RES) + x;
        match face {
            CubeMapFace::PX => self.px[index],
            CubeMapFace::PY => self.py[index],
            CubeMapFace::PZ => self.pz[index],
            CubeMapFace::NX => self.nx[index],
            CubeMapFace::NY => self.ny[index],
            CubeMapFace::NZ => self.nz[index],
        }
    }

    pub fn get(&mut self, coord: DVec3) -> f64 {
        let face = get_face(coord);
        let uv01 = project_direction(&face, coord).unwrap();
        let uv = (uv01 * (RES as f64));
        let mut pixel1 = (uv * (RES as f64)).floor();
        let mut pixel2 = (uv * (RES as f64)).ceil();
        let pixel_fract = (uv * (RES as f64)).fract_gl();

        if (Self::is_out_of_bounds(pixel1.x as isize, pixel1.y as isize)) {
            pixel1.clone_from(&pixel2);
        } else if (Self::is_out_of_bounds(pixel2.x as isize, pixel2.y as isize)) {
            pixel2.clone_from(&pixel1);
        }

        let value11 = self.get_pixel(&face, pixel1.x as usize, pixel1.y as usize);
        let value12 = self.get_pixel(&face, pixel1.x as usize, pixel2.y as usize);
        let value21 = self.get_pixel(&face, pixel2.x as usize, pixel1.y as usize);
        let value22 = self.get_pixel(&face, pixel2.x as usize, pixel2.y as usize);

        let d1 = mix(value11, value21, pixel_fract.x);
        let d2 = mix(value12, value22, pixel_fract.x);

        mix(d1, d2, pixel_fract.y)
    }
}

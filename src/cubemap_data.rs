use crate::math_util::mix;
use glam::{DMat3, DMat4, DVec2, DVec3, DVec4, Mat4, Vec4Swizzles};
use std::cmp::min;
use std::f64::consts::PI;
use std::fmt;

pub struct CubeMapDataLayer {
    res: u16,

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
    let perspective = DMat4::perspective_rh_gl(PI / 2.0, 1.0, 0.01, 10.0);
    let view = match (face) {
        CubeMapFace::PX => DMat4::look_to_rh(
            DVec3::new(0.0, 0.0, 0.0),
            DVec3::new(1.0, 0.0, 0.0),
            DVec3::new(0.0, 1.0, 0.0),
        ),
        CubeMapFace::PY => DMat4::look_to_rh(
            DVec3::new(0.0, 0.0, 0.0),
            DVec3::new(0.0, 1.0, 0.0),
            DVec3::new(0.0, 0.0, -1.0),
        ),
        CubeMapFace::PZ => DMat4::look_to_rh(
            DVec3::new(0.0, 0.0, 0.0),
            DVec3::new(0.0, 0.0, 1.0),
            DVec3::new(0.0, 1.0, 0.0),
        ),
        CubeMapFace::NX => DMat4::look_to_rh(
            DVec3::new(0.0, 0.0, 0.0),
            DVec3::new(-1.0, 0.0, 0.0),
            DVec3::new(0.0, 1.0, 0.0),
        ),
        CubeMapFace::NY => DMat4::look_to_rh(
            DVec3::new(0.0, 0.0, 0.0),
            DVec3::new(0.0, -1.0, 0.0),
            DVec3::new(0.0, 0.0, 1.0),
        ),
        CubeMapFace::NZ => DMat4::look_to_rh(
            DVec3::new(0.0, 0.0, 0.0),
            DVec3::new(0.0, 0.0, -1.0),
            DVec3::new(0.0, 1.0, 0.0),
        ),
    };
    perspective.mul_mat4(&view)
}

fn project_direction(face: &CubeMapFace, coord: DVec3) -> Option<DVec2> {
    let projection = create_projection(face);
    let transformed = projection * DVec4::from((coord, 1.0));
    let res = transformed.xyz() / transformed.w;

    let tolerance = 0.000000000000002;

    if (res.x < -1.0 - tolerance
        || res.x > 1.0 + tolerance
        || res.y < -1.0 - tolerance
        || res.y > 1.0 + tolerance
        || res.z > 1.0 + tolerance)
    {
        return None;
    }
    let mut uv = DVec2::new(-res.x, -res.y) * 0.5 + 0.5;
    // println!("{face}: RES.Z is {}", res.z);
    // match face {
    //     CubeMapFace::NX => {
    //         uv.y = 1.0 - uv.y
    //     }
    //     CubeMapFace::NY => uv.y = 1.0,
    //     CubeMapFace::NZ => uv.y = 1.0,
    //     _ => {}
    // }
    Some(uv * 0.99999999999)
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

impl CubeMapDataLayer {
    pub fn new(res: u16) -> CubeMapDataLayer {
        let res_usize = res as usize;
        CubeMapDataLayer {
            res: res,

            px: vec![0.0; res_usize * res_usize],
            py: vec![0.0; res_usize * res_usize],
            pz: vec![0.0; res_usize * res_usize],

            nx: vec![0.0; res_usize * res_usize],
            ny: vec![0.0; res_usize * res_usize],
            nz: vec![0.0; res_usize * res_usize],
        }
    }

    fn is_out_of_bounds(&self, x: isize, y: isize) -> bool {
        (x as u16) >= self.res || (y as u16) >= self.res || x < 0 || y < 0
    }

    pub fn set_pixel(&mut self, face: &CubeMapFace, x: usize, y: usize, value: f64) {
        let index = y * (self.res as usize) + x;
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
        let uvx = x as f64 / self.res as f64;
        let uvy = y as f64 / self.res as f64;
        let clip = DVec4::new(-(uvx * 2.0 - 1.0), -(uvy * 2.0 - 1.0), 0.1, 1.0);
        let transformed = inv_projection * clip;
        (transformed.xyz() / transformed.w).normalize()
    }

    // TODO if this is to be used, it needs to also do bilinear filtering
    // maybe later
    pub fn set(&mut self, coord: DVec3, value: f64) {
        let face = get_face(coord);
        let uv01 = project_direction(&face, coord).unwrap();
        let uv = (uv01 * (self.res as f64)).floor();
        let index = (uv.y * (self.res as f64) + uv.x) as usize;
        match face {
            CubeMapFace::PX => self.px[index] = value,
            CubeMapFace::PY => self.py[index] = value,
            CubeMapFace::PZ => self.pz[index] = value,
            CubeMapFace::NX => self.nx[index] = value,
            CubeMapFace::NY => self.ny[index] = value,
            CubeMapFace::NZ => self.nz[index] = value,
        }
    }

    // TODO if this is to be used, it needs to also do bilinear filtering
    // maybe later
    pub fn add(&mut self, coord: DVec3, value: f64) {
        let face = get_face(coord);
        let uv01 = project_direction(&face, coord).unwrap();
        let uv = (uv01 * (self.res as f64)).floor();
        let index = (uv.y * (self.res as f64) + uv.x) as usize;
        match face {
            CubeMapFace::PX => self.px[index] += value,
            CubeMapFace::PY => self.py[index] += value,
            CubeMapFace::PZ => self.pz[index] += value,
            CubeMapFace::NX => self.nx[index] += value,
            CubeMapFace::NY => self.ny[index] += value,
            CubeMapFace::NZ => self.nz[index] += value,
        }
    }

    pub fn get_pixel(&self, face: &CubeMapFace, x: usize, y: usize) -> f64 {
        let index = min(
            y * (self.res as usize) + x,
            (self.res as usize) * (self.res as usize) - 1,
        );
        match face {
            CubeMapFace::PX => self.px[index],
            CubeMapFace::PY => self.py[index],
            CubeMapFace::PZ => self.pz[index],
            CubeMapFace::NX => self.nx[index],
            CubeMapFace::NY => self.ny[index],
            CubeMapFace::NZ => self.nz[index],
        }
    }

    pub fn get(&self, coord: DVec3) -> f64 {
        let face = get_face(coord);
        let uv01 = project_direction(&face, coord).unwrap();
        let uv = (uv01 * (self.res as f64));
        let mut pixel1 = uv.floor();
        let mut pixel2 = uv.ceil();
        let pixel_fract = uv.fract_gl();

        if (self.is_out_of_bounds(pixel1.x as isize, pixel1.y as isize)) {
            pixel1.clone_from(&pixel2);
        } else if (self.is_out_of_bounds(pixel2.x as isize, pixel2.y as isize)) {
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

    pub fn get_normal(
        &self,
        dir: DVec3,
        dxrange: f64,
        sphere_radius: f64,
        terrain_height: f64,
    ) -> DVec3 {
        let mut tangdir = DMat3::from_axis_angle(DVec3::new(0.0, 1.0, 0.0), PI) * dir;
        let bitangdir = tangdir.cross(dir).normalize();
        tangdir = dir.cross(bitangdir).normalize();
        let normrotmat1 = DMat3::from_axis_angle(tangdir, dxrange);
        let normrotmat2 = DMat3::from_axis_angle(bitangdir, dxrange);
        let dir2 = normrotmat1 * dir;
        let dir3 = normrotmat2 * dir;
        let p1 = dir * (sphere_radius + self.get(dir) * terrain_height);
        let p2 = dir2 * (sphere_radius + self.get(dir2) * terrain_height);
        let p3 = dir3 * (sphere_radius + self.get(dir3) * terrain_height);
        let a = (p3 - p1).normalize();
        let b = (p2 - p1).normalize();
        a.cross(b).normalize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reprojection_px() {
        const RES: u16 = 128;
        let mut cube_map: CubeMapDataLayer = CubeMapDataLayer::new(RES);

        let dir = cube_map.pixel_coords_to_direction(&CubeMapFace::PX, 32, 32);
        let uv = project_direction(&CubeMapFace::PX, dir).unwrap();
        let pixels = (uv * (RES as f64)).floor();
        println!("{dir} {uv} {}", { uv * (RES as f64) });
        assert_eq!(pixels.x, 31.0);
        assert_eq!(pixels.y, 31.0);
    }

    #[test]
    fn test_reprojection_py() {
        const RES: u16 = 128;
        let mut cube_map: CubeMapDataLayer = CubeMapDataLayer::new(RES);

        let dir = cube_map.pixel_coords_to_direction(&CubeMapFace::PY, 32, 32);
        let uv = project_direction(&CubeMapFace::PY, dir).unwrap();
        let pixels = (uv * (RES as f64)).floor();
        println!("{dir} {uv} {}", { uv * (RES as f64) });
        assert_eq!(pixels.x, 31.0);
        assert_eq!(pixels.y, 31.0);
    }

    #[test]
    fn test_reprojection_pz() {
        const RES: u16 = 128;
        let mut cube_map: CubeMapDataLayer = CubeMapDataLayer::new(RES);

        let dir = cube_map.pixel_coords_to_direction(&CubeMapFace::PZ, 32, 32);
        let uv = project_direction(&CubeMapFace::PZ, dir).unwrap();
        let pixels = (uv * (RES as f64)).floor();
        println!("{dir} {uv} {}", { uv * (RES as f64) });
        assert_eq!(pixels.x, 31.0);
        assert_eq!(pixels.y, 31.0);
    }

    #[test]
    fn test_reprojection_nx() {
        const RES: u16 = 128;
        let mut cube_map: CubeMapDataLayer = CubeMapDataLayer::new(RES);

        let dir = cube_map.pixel_coords_to_direction(&CubeMapFace::NX, 32, 32);
        let uv = project_direction(&CubeMapFace::NX, dir).unwrap();
        let pixels = (uv * (RES as f64)).floor();
        println!("{dir} {uv} {}", { uv * (RES as f64) });
        assert_eq!(pixels.x, 31.0);
        assert_eq!(pixels.y, 31.0);
    }

    #[test]
    fn test_reprojection_ny() {
        const RES: u16 = 128;
        let mut cube_map: CubeMapDataLayer = CubeMapDataLayer::new(RES);

        let dir = cube_map.pixel_coords_to_direction(&CubeMapFace::NY, 32, 32);
        let uv = project_direction(&CubeMapFace::NY, dir).unwrap();
        let pixels = (uv * (RES as f64)).floor();
        println!("{dir} {uv} {}", { uv * (RES as f64) });
        assert_eq!(pixels.x, 31.0);
        assert_eq!(pixels.y, 31.0);
    }

    #[test]
    fn test_reprojection_nz() {
        const RES: u16 = 128;
        let mut cube_map: CubeMapDataLayer = CubeMapDataLayer::new(RES);

        let dir = cube_map.pixel_coords_to_direction(&CubeMapFace::NZ, 32, 32);
        let uv = project_direction(&CubeMapFace::NZ, dir).unwrap();
        let pixels = (uv * (RES as f64)).floor();
        println!("{dir} {uv} {}", { uv * (RES as f64) });
        assert_eq!(pixels.x, 31.0);
        assert_eq!(pixels.y, 31.0);
    }

    #[test]
    fn test_set_pixel() {
        const RES: u16 = 128;
        let mut cube_map: CubeMapDataLayer = CubeMapDataLayer::new(RES);

        let faces = [
            CubeMapFace::PX,
            CubeMapFace::PY,
            CubeMapFace::PZ,
            CubeMapFace::NX,
            CubeMapFace::NY,
            CubeMapFace::NZ,
        ];

        faces.iter().for_each(|face| {
            cube_map.set_pixel(face, 32, 32, 123.0);

            cube_map.set_pixel(face, 31, 32, 64.0);
            cube_map.set_pixel(face, 32, 31, 64.0);
            cube_map.set_pixel(face, 31, 31, 64.0);

            cube_map.set_pixel(face, 33, 32, 64.0);
            cube_map.set_pixel(face, 32, 33, 64.0);
            cube_map.set_pixel(face, 33, 33, 64.0);

            let dir = cube_map.pixel_coords_to_direction(face, 32, 32);

            let readback = cube_map.get(dir).round();
            println!("{face}: {readback}");
            assert_eq!(readback, 123.0);
        })
    }
}

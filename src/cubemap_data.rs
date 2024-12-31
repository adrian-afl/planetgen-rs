use crate::math_util::mix;
use glam::{DMat3, DMat4, DVec2, DVec3, DVec4, Mat4, Vec4Swizzles};
use std::cell::RefCell;
use std::cmp::{max, min};
use std::f64::consts::PI;
use std::fmt;
use std::sync::{Arc, Mutex};

pub struct CubeMapDataLayer<Data> {
    pub res: u16,

    px: Arc<Mutex<Vec<Data>>>,
    py: Arc<Mutex<Vec<Data>>>,
    pz: Arc<Mutex<Vec<Data>>>,
    nx: Arc<Mutex<Vec<Data>>>,
    ny: Arc<Mutex<Vec<Data>>>,
    nz: Arc<Mutex<Vec<Data>>>,
}

#[derive(Clone)]
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

impl<Data: Clone> CubeMapDataLayer<Data> {
    pub fn new(res: u16, initializer: Data) -> CubeMapDataLayer<Data> {
        let res_usize = res as usize;
        CubeMapDataLayer {
            res: res,

            px: Arc::new(Mutex::from(vec![
                initializer.clone();
                res_usize * res_usize
            ])),
            py: Arc::new(Mutex::from(vec![
                initializer.clone();
                res_usize * res_usize
            ])),
            pz: Arc::new(Mutex::from(vec![
                initializer.clone();
                res_usize * res_usize
            ])),
            nx: Arc::new(Mutex::from(vec![
                initializer.clone();
                res_usize * res_usize
            ])),
            ny: Arc::new(Mutex::from(vec![
                initializer.clone();
                res_usize * res_usize
            ])),
            nz: Arc::new(Mutex::from(vec![
                initializer.clone();
                res_usize * res_usize
            ])),
        }
    }

    fn is_out_of_bounds(&self, x: isize, y: isize) -> bool {
        (x as u16) >= self.res || (y as u16) >= self.res || x < 0 || y < 0
    }

    pub fn set_pixel(&self, face: &CubeMapFace, x: usize, y: usize, value: Data) {
        let index = y * (self.res as usize) + x;
        match face {
            CubeMapFace::PX => self.px.lock().unwrap()[index] = value,
            CubeMapFace::PY => self.py.lock().unwrap()[index] = value,
            CubeMapFace::PZ => self.pz.lock().unwrap()[index] = value,
            CubeMapFace::NX => self.nx.lock().unwrap()[index] = value,
            CubeMapFace::NY => self.ny.lock().unwrap()[index] = value,
            CubeMapFace::NZ => self.nz.lock().unwrap()[index] = value,
        }
    }

    pub fn get_mutable_face(&self, face: &CubeMapFace) -> Arc<Mutex<Vec<Data>>> {
        match face {
            CubeMapFace::PX => self.px.clone(),
            CubeMapFace::PY => self.py.clone(),
            CubeMapFace::PZ => self.pz.clone(),
            CubeMapFace::NX => self.nx.clone(),
            CubeMapFace::NY => self.ny.clone(),
            CubeMapFace::NZ => self.nz.clone(),
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

    pub fn get_smallest_pixel_distance(&self) -> f64 {
        let a = self.pixel_coords_to_direction(
            &CubeMapFace::PX,
            self.res as usize / 2,
            self.res as usize / 2,
        );
        let b = self.pixel_coords_to_direction(
            &CubeMapFace::PX,
            self.res as usize / 2 + 1,
            self.res as usize / 2 + 1,
        );
        a.distance(b)
    }

    pub fn get_biggest_pixel_distance(&self) -> f64 {
        let a = self.pixel_coords_to_direction(&CubeMapFace::PX, 0, 0);
        let b = self.pixel_coords_to_direction(&CubeMapFace::PX, 1, 1);
        a.distance(b)
    }

    pub fn get_pixel_distance_for_dir(&self, coord: DVec3) -> f64 {
        let face = get_face(coord);
        let uv01 = project_direction(&face, coord).unwrap();
        let uv = (uv01 * (self.res as f64)).floor();
        let a = self.pixel_coords_to_direction(&CubeMapFace::PX, uv.x as usize, uv.y as usize);
        let b =
            self.pixel_coords_to_direction(&CubeMapFace::PX, uv.x as usize + 1, uv.y as usize + 1);
        a.distance(b)
    }

    // TODO if this is to be used, it needs to also do bilinear filtering
    // maybe later
    pub fn set(&self, coord: DVec3, value: Data) {
        let face = get_face(coord);
        let uv01 = project_direction(&face, coord).unwrap();
        let uv = (uv01 * (self.res as f64)).floor();
        let index = (uv.y * (self.res as f64) + uv.x) as usize;
        match face {
            CubeMapFace::PX => self.px.lock().unwrap()[index] = value,
            CubeMapFace::PY => self.py.lock().unwrap()[index] = value,
            CubeMapFace::PZ => self.pz.lock().unwrap()[index] = value,
            CubeMapFace::NX => self.nx.lock().unwrap()[index] = value,
            CubeMapFace::NY => self.ny.lock().unwrap()[index] = value,
            CubeMapFace::NZ => self.nz.lock().unwrap()[index] = value,
        }
    }

    pub fn get_pixel(&self, face: &CubeMapFace, x: usize, y: usize) -> Data {
        let index = min(
            y * (self.res as usize) + x,
            (self.res as usize) * (self.res as usize) - 1,
        );
        match face {
            CubeMapFace::PX => self.px.lock().unwrap()[index].clone(),
            CubeMapFace::PY => self.py.lock().unwrap()[index].clone(),
            CubeMapFace::PZ => self.pz.lock().unwrap()[index].clone(),
            CubeMapFace::NX => self.nx.lock().unwrap()[index].clone(),
            CubeMapFace::NY => self.ny.lock().unwrap()[index].clone(),
            CubeMapFace::NZ => self.nz.lock().unwrap()[index].clone(),
        }
    }

    pub fn get(&self, coord: DVec3) -> Data {
        let face = get_face(coord);
        let uv01 = project_direction(&face, coord).unwrap();
        let uv = (uv01 * (self.res as f64));
        let mut pixel = uv.floor();
        if (pixel.x < 0.0) {
            pixel.x = 0.0;
        }
        if (pixel.x >= (self.res - 1) as f64) {
            pixel.x = (self.res - 1) as f64;
        }
        if (pixel.y < 0.0) {
            pixel.y = 0.0;
        }
        if (pixel.y >= (self.res - 1) as f64) {
            pixel.y = (self.res - 1) as f64;
        }
        self.get_pixel(&face, pixel.x as usize, pixel.y as usize)
    }
}

impl CubeMapDataLayer<f64> {
    // TODO if this is to be used, it needs to also do bilinear filtering
    // maybe later
    pub fn add(&self, coord: DVec3, value: f64) {
        let face = get_face(coord);
        let uv01 = project_direction(&face, coord).unwrap();
        let uv = (uv01 * (self.res as f64)).floor();
        let index = (uv.y * (self.res as f64) + uv.x) as usize;
        match face {
            CubeMapFace::PX => self.px.lock().unwrap()[index] += value,
            CubeMapFace::PY => self.py.lock().unwrap()[index] += value,
            CubeMapFace::PZ => self.pz.lock().unwrap()[index] += value,
            CubeMapFace::NX => self.nx.lock().unwrap()[index] += value,
            CubeMapFace::NY => self.ny.lock().unwrap()[index] += value,
            CubeMapFace::NZ => self.nz.lock().unwrap()[index] += value,
        }
    }

    pub fn add_pixel(&self, face: &CubeMapFace, x: usize, y: usize, value: f64) {
        let index = min(
            y * (self.res as usize) + x,
            (self.res as usize) * (self.res as usize) - 1,
        );
        match face {
            CubeMapFace::PX => self.px.lock().unwrap()[index] += value,
            CubeMapFace::PY => self.py.lock().unwrap()[index] += value,
            CubeMapFace::PZ => self.pz.lock().unwrap()[index] += value,
            CubeMapFace::NX => self.nx.lock().unwrap()[index] += value,
            CubeMapFace::NY => self.ny.lock().unwrap()[index] += value,
            CubeMapFace::NZ => self.nz.lock().unwrap()[index] += value,
        }
    }

    pub fn add_bilinear(&self, coord: DVec3, value: f64) {
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

        // I am REALLY not sure - TODO check it later

        let pixcoord11 = (pixel1.x as usize, pixel1.y as usize);
        let pixcoord12 = (pixel1.x as usize, pixel2.y as usize);
        let pixcoord21 = (pixel2.x as usize, pixel1.y as usize);
        let pixcoord22 = (pixel2.x as usize, pixel2.y as usize);

        self.add_pixel(&face, pixcoord11.0, pixcoord11.1, value * pixel_fract.x);
        self.add_pixel(
            &face,
            pixcoord12.0,
            pixcoord12.1,
            value * (1.0 - pixel_fract.x),
        );

        self.add_pixel(&face, pixcoord21.0, pixcoord21.1, value * pixel_fract.y);
        self.add_pixel(
            &face,
            pixcoord22.0,
            pixcoord22.1,
            value * (1.0 - pixel_fract.y),
        );
    }

    pub fn get_bilinear(&self, coord: DVec3) -> f64 {
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

    pub fn get_normal(&self, dir: DVec3, dxrange: f64) -> DVec3 {
        let dir = dir.normalize();

        let tangdir = if dir.y.abs() < 0.99 {
            DVec3::new(0.0, 1.0, 0.0).cross(dir).normalize()
        } else {
            DVec3::new(1.0, 0.0, 0.0).cross(dir).normalize()
        };
        let bitangdir = dir.cross(tangdir).normalize();

        let dir1 = (dir + tangdir * dxrange).normalize();
        let dir2 = (dir + bitangdir * dxrange).normalize();
        let dir3 = (dir - tangdir * dxrange).normalize();
        let dir4 = (dir - bitangdir * dxrange).normalize();

        let p1 = dir1 * self.get_bilinear(dir1);
        let p2 = dir2 * self.get_bilinear(dir2);
        let p3 = dir3 * self.get_bilinear(dir3);
        let p4 = dir4 * self.get_bilinear(dir4);

        let n1 = (p2 - p1).cross(p3 - p1);
        let n2 = (p3 - p1).cross(p4 - p1);

        (n1 + n2).normalize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reprojection_px() {
        const RES: u16 = 128;
        let mut cube_map: CubeMapDataLayer<f64> = CubeMapDataLayer::new(RES, 0.0);

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
        let mut cube_map: CubeMapDataLayer<f64> = CubeMapDataLayer::new(RES, 0.0);

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
        let mut cube_map: CubeMapDataLayer<f64> = CubeMapDataLayer::new(RES, 0.0);

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
        let mut cube_map: CubeMapDataLayer<f64> = CubeMapDataLayer::new(RES, 0.0);

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
        let mut cube_map: CubeMapDataLayer<f64> = CubeMapDataLayer::new(RES, 0.0);

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
        let mut cube_map: CubeMapDataLayer<f64> = CubeMapDataLayer::new(RES, 0.0);

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
        let mut cube_map: CubeMapDataLayer<f64> = CubeMapDataLayer::new(RES, 0.0);

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

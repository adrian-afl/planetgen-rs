use crate::base_icosphere::get_base_icosphere;
use crate::cubemap_data::CubeMapDataLayer;
use glam::DVec3;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub type Triangle = [DVec3; 3];

fn subdivide_triangle(tri: &Triangle) -> [Triangle; 4] {
    let half_edge_a = tri[0].lerp(tri[1], 0.5);
    let half_edge_b = tri[1].lerp(tri[2], 0.5);
    let half_edge_c = tri[2].lerp(tri[0], 0.5);

    [
        [tri[0], half_edge_a, half_edge_c],
        [tri[1], half_edge_b, half_edge_a],
        [tri[2], half_edge_c, half_edge_b],
        [half_edge_a, half_edge_b, half_edge_c],
    ]
}

fn normalize_triangle(tri: &mut Triangle) {
    tri[0] = tri[0].normalize();
    tri[1] = tri[1].normalize();
    tri[2] = tri[2].normalize();
}

fn scale_vector<const RES: usize>(
    v: DVec3,
    input: &CubeMapDataLayer<RES>,
    scale: f64,
    terrain_scale: f64,
) -> DVec3 {
    let value = input.get(v);
    v * (scale + terrain_scale * value)
}

fn scale_triangle<const RES: usize>(
    tri: &mut Triangle,
    input: &CubeMapDataLayer<RES>,
    scale: f64,
    terrain_scale: f64,
) {
    tri[0] = scale_vector(tri[0], input, scale, terrain_scale);
    tri[1] = scale_vector(tri[1], input, scale, terrain_scale);
    tri[2] = scale_vector(tri[2], input, scale, terrain_scale);
}

fn get_triangle_normal(tri: &mut Triangle) -> DVec3 {
    (tri[1] - tri[0]).cross(tri[2] - tri[0]).normalize()
}

fn write_vector(file: &mut File, v: DVec3, n: DVec3) {
    file.write(&(v.x as f32).to_le_bytes());
    file.write(&(v.y as f32).to_le_bytes());
    file.write(&(v.z as f32).to_le_bytes());

    file.write(&(n.x as f32).to_le_bytes());
    file.write(&(n.y as f32).to_le_bytes());
    file.write(&(n.z as f32).to_le_bytes());
}

fn write_triangle(file: &mut File, tri: &mut Triangle) {
    let normal = get_triangle_normal(tri);
    write_vector(file, tri[0], normal);
    write_vector(file, tri[1], normal);
    write_vector(file, tri[2], normal);
}

pub fn generate_icosphere_raw<const RES: usize>(
    output: &Path,
    input: &CubeMapDataLayer<RES>,
    scale: f64,
    terrain_scale: f64,
) {
    let base = get_base_icosphere();

    let mut file = File::create(output).expect("create failed");
    // let mut first_subdivision: Vec<Triangle> = vec![];
    base.map(|triangle| {
        let mut triangles = subdivide_triangle(&triangle);
        triangles.iter_mut().for_each(|t0| {
            let mut triangles = subdivide_triangle(t0);
            triangles.iter_mut().for_each(|t1| {
                let mut triangles = subdivide_triangle(t1);
                triangles.iter_mut().for_each(|t2| {
                    let mut triangles = subdivide_triangle(t2);
                    triangles.iter_mut().for_each(|t| {
                        normalize_triangle(t);
                        scale_triangle(t, input, scale, terrain_scale);
                        write_triangle(&mut file, t);
                    });
                });
            });
        });
    });

    file.flush().unwrap();
}

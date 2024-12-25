use crate::base_icosphere::get_base_icosphere;
use crate::cubemap_data::CubeMapDataLayer;
use glam::DVec3;
use rayon::iter::IndexedParallelIterator;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelIterator;
use std::fs::File;
use std::io::Write;

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

fn subdivide_triangle_multiple(tri: Triangle, count: i32) -> Vec<Triangle> {
    let mut triangles = vec![tri];
    for i in 0..count {
        let mut tmp: Vec<Triangle> = vec![];
        for t in 0..triangles.len() {
            let cur = subdivide_triangle(&triangles[t]);
            tmp.push(cur[0]);
            tmp.push(cur[1]);
            tmp.push(cur[2]);
            tmp.push(cur[3]);
        }
        triangles = tmp;
    }
    triangles
}

fn normalize_triangle(tri: &Triangle) -> Triangle {
    [tri[0].normalize(), tri[1].normalize(), tri[2].normalize()]
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
    tri: &Triangle,
    input: &CubeMapDataLayer<RES>,
    scale: f64,
    terrain_scale: f64,
) -> Triangle {
    [
        scale_vector(tri[0], input, scale, terrain_scale),
        scale_vector(tri[1], input, scale, terrain_scale),
        scale_vector(tri[2], input, scale, terrain_scale),
    ]
}

fn get_triangle_normal(tri: &Triangle) -> DVec3 {
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

fn write_triangle(file: &mut File, tri: &Triangle) {
    let normal = get_triangle_normal(tri);
    write_vector(file, tri[0], normal);
    write_vector(file, tri[1], normal);
    write_vector(file, tri[2], normal);
}

pub fn generate_icosphere_raw<const RES: usize>(
    outputDir: &str,
    input: &CubeMapDataLayer<RES>,
    scale: f64,
    terrain_scale: f64,
) {
    let base = get_base_icosphere();

    // let mut first_subdivision: Vec<Triangle> = vec![];
    base.into_iter()
        .enumerate()
        .for_each(|(index_main, triangle)| {
            let mut level0 = subdivide_triangle_multiple(triangle, 2);

            level0.into_par_iter().enumerate().for_each(|(index, t)| {
                // let mut level0file =
                //     File::create(outputDir.to_owned() + "/" + index.to_string().as_str() + ".l0.raw")
                //         .expect("create failed");
                let mut level1file = File::create(
                    outputDir.to_owned()
                        + "/"
                        + (index_main).to_string().as_str()
                        + "."
                        + (index).to_string().as_str()
                        + ".l1.raw",
                )
                .expect("create failed");
                let mut level2file = File::create(
                    outputDir.to_owned()
                        + "/"
                        + (index_main).to_string().as_str()
                        + "."
                        + (index).to_string().as_str()
                        + ".l2.raw",
                )
                .expect("create failed");
                let mut level3file = File::create(
                    outputDir.to_owned()
                        + "/"
                        + (index_main).to_string().as_str()
                        + "."
                        + (index).to_string().as_str()
                        + ".l3.raw",
                )
                .expect("create failed");

                let mut level1 = subdivide_triangle_multiple(t, 3);
                let mut level2 = subdivide_triangle_multiple(t, 5);
                let mut level3 = subdivide_triangle_multiple(t, 7);

                // let t = normalize_triangle(&t);
                // let t = scale_triangle(&t, input, scale, terrain_scale);
                // write_triangle(&mut level0file, &t);

                level1.iter_mut().for_each(|t| {
                    let t = normalize_triangle(&t);
                    let t = scale_triangle(&t, input, scale, terrain_scale);
                    write_triangle(&mut level1file, &t);
                });

                level2.iter_mut().for_each(|t| {
                    let t = normalize_triangle(&t);
                    let t = scale_triangle(&t, input, scale, terrain_scale);
                    write_triangle(&mut level2file, &t);
                });

                level3.iter_mut().for_each(|t| {
                    let t = normalize_triangle(&t);
                    let t = scale_triangle(&t, input, scale, terrain_scale);
                    write_triangle(&mut level3file, &t);
                });
                level1file.flush().unwrap();
                level2file.flush().unwrap();
                level3file.flush().unwrap();
            });

            // let mut triangles = subdivide_triangle(&triangle);
            // triangles.iter_mut().for_each(|t0| {
            //     let mut triangles = subdivide_triangle(t0);
            //     triangles.iter_mut().for_each(|t1| {
            //         let mut triangles = subdivide_triangle(t1);
            //         triangles.iter_mut().for_each(|t2| {
            //             let mut triangles = subdivide_triangle(t2);
            //             triangles.iter_mut().for_each(|t| {
            //                 normalize_triangle(t);
            //                 scale_triangle(t, input, scale, terrain_scale);
            //                 write_triangle(&mut file, t);
            //             });
            //         });
            //     });
            // });
        });
}

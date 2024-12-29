use crate::base_icosphere::get_base_icosphere;
use crate::cubemap_data::CubeMapDataLayer;
use crate::generate_terrain::InterpolatedBiomeData;
use deflate::write::DeflateEncoder;
use deflate::Compression;
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

fn subdivide_triangle_multiple(tri: Triangle, count: u8) -> Vec<Triangle> {
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

fn scale_vector(v: DVec3, input: &CubeMapDataLayer<f64>) -> DVec3 {
    v * input.get_bilinear(v)
}

fn scale_triangle(tri: &Triangle, input: &CubeMapDataLayer<f64>) -> Triangle {
    [
        scale_vector(tri[0], input),
        scale_vector(tri[1], input),
        scale_vector(tri[2], input),
    ]
}

fn get_triangle_center(tri: &Triangle, scale: f64) -> DVec3 {
    ((tri[0] + tri[1] + tri[2]) / 3.0).normalize() * scale
}

fn translate_triangle(tri: &Triangle, translation: DVec3) -> Triangle {
    [
        tri[0] + translation,
        tri[1] + translation,
        tri[2] + translation,
    ]
}

fn get_triangle_normal(tri: &Triangle) -> DVec3 {
    (tri[1] - tri[0]).cross(tri[2] - tri[0]).normalize()
}

/*
Terrain layout is:
    position: vec3
    normal: vec3
    color: vec3
    roughness: float
*/
fn write_vector_terrain(
    file: &mut DeflateEncoder<File>,
    v: DVec3,
    n: DVec3,
    interpolated_biome_data: InterpolatedBiomeData,
) {
    file.write(&(v.x as f32).to_le_bytes())
        .expect("Write failed");
    file.write(&(v.y as f32).to_le_bytes())
        .expect("Write failed");
    file.write(&(v.z as f32).to_le_bytes())
        .expect("Write failed");

    file.write(&(n.x as f32).to_le_bytes())
        .expect("Write failed");
    file.write(&(n.y as f32).to_le_bytes())
        .expect("Write failed");
    file.write(&(n.z as f32).to_le_bytes())
        .expect("Write failed");

    file.write(&(interpolated_biome_data.color.x as f32).to_le_bytes())
        .expect("Write failed");
    file.write(&(interpolated_biome_data.color.y as f32).to_le_bytes())
        .expect("Write failed");
    file.write(&(interpolated_biome_data.color.z as f32).to_le_bytes())
        .expect("Write failed");

    file.write(&(interpolated_biome_data.roughness as f32).to_le_bytes())
        .expect("Write failed");
}

fn write_triangle_terrain(
    height_data: &CubeMapDataLayer<f64>,
    biome_data: &CubeMapDataLayer<InterpolatedBiomeData>,
    file: &mut DeflateEncoder<File>,
    tri: &Triangle,
) {
    let vec0dir = tri[0].clone().normalize();
    let vec1dir = tri[1].clone().normalize();
    let vec2dir = tri[2].clone().normalize();
    write_vector_terrain(
        file,
        tri[0],
        height_data.get_normal(vec0dir, height_data.get_pixel_distance_for_dir(vec0dir)),
        biome_data.get(vec0dir),
    );
    write_vector_terrain(
        file,
        tri[1],
        height_data.get_normal(vec1dir, height_data.get_pixel_distance_for_dir(vec1dir)),
        biome_data.get(vec1dir),
    );
    write_vector_terrain(
        file,
        tri[2],
        height_data.get_normal(vec2dir, height_data.get_pixel_distance_for_dir(vec2dir)),
        biome_data.get(vec2dir),
    );
}

/*
Water layout is just:
    position: vec3
*/
fn write_vector_water(file: &mut DeflateEncoder<File>, v: DVec3) {
    file.write(&(v.x as f32).to_le_bytes())
        .expect("Write failed");
    file.write(&(v.y as f32).to_le_bytes())
        .expect("Write failed");
    file.write(&(v.z as f32).to_le_bytes())
        .expect("Write failed");
}

fn write_triangle_water(
    height_data: &CubeMapDataLayer<f64>,
    file: &mut DeflateEncoder<File>,
    tri: &Triangle,
) {
    write_vector_water(file, tri[0]);
    write_vector_water(file, tri[1]);
    write_vector_water(file, tri[2]);
}

pub fn generate_icosphere_raw(
    output_dir: &str,
    height_data: &CubeMapDataLayer<f64>,
    biome_data: Option<&CubeMapDataLayer<InterpolatedBiomeData>>,
    sphere_radius: f64,
    subdivide_initial: u8,
    subdivide_level1: u8,
    subdivide_level2: u8,
    subdivide_level3: u8,
) {
    let base = get_base_icosphere();

    let mut metadata_file =
        File::create(output_dir.to_owned() + "/metadata.ini").expect("create failed");

    base.into_iter()
        .enumerate()
        .for_each(|(index_main, triangle)| {
            println!("{index_main}/{}", base.len());
            let mut level0 = subdivide_triangle_multiple(triangle, subdivide_initial);

            // not parallel to not worry about borrow checker
            level0
                .clone()
                .into_iter()
                .enumerate()
                .for_each(|(index, t)| {
                    let part_center = get_triangle_center(&t, sphere_radius);
                    let data = format!(
                        "{index_main}-{index}={},{},{}\n",
                        part_center.x, part_center.y, part_center.z
                    );
                    metadata_file.write(data.as_bytes()).expect("Write failed");
                });

            level0.into_par_iter().enumerate().for_each(|(index, t)| {
                let mut level1file = DeflateEncoder::new(
                    File::create(
                        output_dir.to_owned()
                            + "/"
                            + (index_main).to_string().as_str()
                            + "-"
                            + (index).to_string().as_str()
                            + ".l1.raw",
                    )
                    .expect("create failed"),
                    Compression::Best,
                );

                let mut level2file = DeflateEncoder::new(
                    File::create(
                        output_dir.to_owned()
                            + "/"
                            + (index_main).to_string().as_str()
                            + "-"
                            + (index).to_string().as_str()
                            + ".l2.raw",
                    )
                    .expect("create failed"),
                    Compression::Best,
                );

                let mut level3file = DeflateEncoder::new(
                    File::create(
                        output_dir.to_owned()
                            + "/"
                            + (index_main).to_string().as_str()
                            + "-"
                            + (index).to_string().as_str()
                            + ".l3.raw",
                    )
                    .expect("create failed"),
                    Compression::Best,
                );

                let part_center = get_triangle_center(&t, sphere_radius);

                let mut level1 = subdivide_triangle_multiple(t, subdivide_level1);
                let mut level2 = subdivide_triangle_multiple(t, subdivide_level2);
                let mut level3 = subdivide_triangle_multiple(t, subdivide_level3);

                level1.iter_mut().for_each(|t| {
                    let t = normalize_triangle(&t);
                    let t = scale_triangle(&t, height_data);
                    let t = translate_triangle(&t, -part_center);
                    match biome_data {
                        None => write_triangle_water(&height_data, &mut level1file, &t),
                        Some(biome_data) => {
                            write_triangle_terrain(&height_data, &biome_data, &mut level1file, &t)
                        }
                    }
                });

                level2.iter_mut().for_each(|t| {
                    let t = normalize_triangle(&t);
                    let t = scale_triangle(&t, height_data);
                    let t = translate_triangle(&t, -part_center);
                    match biome_data {
                        None => write_triangle_water(&height_data, &mut level2file, &t),
                        Some(biome_data) => {
                            write_triangle_terrain(&height_data, &biome_data, &mut level2file, &t)
                        }
                    }
                });

                level3.iter_mut().for_each(|t| {
                    let t = normalize_triangle(&t);
                    let t = scale_triangle(&t, height_data);
                    let t = translate_triangle(&t, -part_center);
                    match biome_data {
                        None => write_triangle_water(&height_data, &mut level3file, &t),
                        Some(biome_data) => {
                            write_triangle_terrain(&height_data, &biome_data, &mut level3file, &t)
                        }
                    }
                });
                level1file.flush().unwrap();
                level2file.flush().unwrap();
                level3file.flush().unwrap();
            });
        });
    metadata_file.flush().unwrap();
}

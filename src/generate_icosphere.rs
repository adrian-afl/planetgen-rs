use crate::base_icosphere::get_base_icosphere;
use crate::cubemap_data::CubeMapDataLayer;
use crate::generate_terrain::InterpolatedBiomeData;
use flate2::write::ZlibEncoder;
use flate2::Compression;
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
    global_index: uint32
*/
fn write_vector_terrain(
    file: &mut dyn Write,
    v: DVec3,
    n: DVec3,
    interpolated_biome_data: InterpolatedBiomeData,
    global_index: u32,
) {
    file.write_all(&(v.x as f32).to_le_bytes())
        .expect("Write failed");
    file.write_all(&(v.y as f32).to_le_bytes())
        .expect("Write failed");
    file.write_all(&(v.z as f32).to_le_bytes())
        .expect("Write failed");

    file.write_all(&((n.x * 127.0) as u8).to_le_bytes())
        .expect("Write failed");
    file.write_all(&((n.y * 127.0) as u8).to_le_bytes())
        .expect("Write failed");
    file.write_all(&((n.z * 127.0) as u8).to_le_bytes())
        .expect("Write failed");
    file.write_all(&((0) as u8).to_le_bytes())
        .expect("Write failed");

    file.write_all(&((interpolated_biome_data.color.x * 255.0) as u8).to_le_bytes())
        .expect("Write failed");
    file.write_all(&((interpolated_biome_data.color.y * 255.0) as u8).to_le_bytes())
        .expect("Write failed");
    file.write_all(&((interpolated_biome_data.color.z * 255.0) as u8).to_le_bytes())
        .expect("Write failed");
    file.write_all(&((interpolated_biome_data.roughness * 255.0) as u8).to_le_bytes())
        .expect("Write failed");

    file.write_all(&(global_index as u16).to_le_bytes())
        .expect("Write failed");
    file.write_all(&((0) as u16).to_le_bytes())
        .expect("Write failed");
}

fn write_triangle_terrain(
    height_data: &CubeMapDataLayer<f64>,
    biome_data: &CubeMapDataLayer<InterpolatedBiomeData>,
    file: &mut dyn Write,
    tri: &Triangle,
    norm_tri: &Triangle,
    global_index: u32,
) {
    write_vector_terrain(
        file,
        tri[0],
        height_data.get_normal(
            norm_tri[0],
            height_data.get_pixel_distance_for_dir(norm_tri[0]),
        ),
        biome_data.get(norm_tri[0]),
        global_index,
    );
    write_vector_terrain(
        file,
        tri[1],
        height_data.get_normal(
            norm_tri[1],
            height_data.get_pixel_distance_for_dir(norm_tri[1]),
        ),
        biome_data.get(norm_tri[1]),
        global_index,
    );
    write_vector_terrain(
        file,
        tri[2],
        height_data.get_normal(
            norm_tri[2],
            height_data.get_pixel_distance_for_dir(norm_tri[2]),
        ),
        biome_data.get(norm_tri[2]),
        global_index,
    );
}

/*
Water layout is just:
    position: vec3
    global_index: uint32
*/
fn write_vector_water(file: &mut dyn Write, v: DVec3, global_index: u32) {
    file.write_all(&(v.x as f32).to_le_bytes())
        .expect("Write failed");
    file.write_all(&(v.y as f32).to_le_bytes())
        .expect("Write failed");
    file.write_all(&(v.z as f32).to_le_bytes())
        .expect("Write failed");

    file.write_all(&(global_index as u16).to_le_bytes())
        .expect("Write failed");
    file.write_all(&((0) as u16).to_le_bytes())
        .expect("Write failed");
}

fn write_triangle_water(file: &mut dyn Write, tri: &Triangle, global_index: u32) {
    write_vector_water(file, tri[0], global_index);
    write_vector_water(file, tri[1], global_index);
    write_vector_water(file, tri[2], global_index);
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

            let level0_len = level0.len();
            level0.into_par_iter().enumerate().for_each(|(index, t)| {
                let mut level1file = brotli::CompressorWriter::new(
                    File::create(
                        output_dir.to_owned()
                            + "/"
                            + (index_main).to_string().as_str()
                            + "-"
                            + (index).to_string().as_str()
                            + ".l1.raw",
                    )
                    .expect("create failed"),
                    40960,
                    11,
                    21,
                );

                let mut level2file = brotli::CompressorWriter::new(
                    File::create(
                        output_dir.to_owned()
                            + "/"
                            + (index_main).to_string().as_str()
                            + "-"
                            + (index).to_string().as_str()
                            + ".l2.raw",
                    )
                    .expect("create failed"),
                    40960,
                    11,
                    21,
                );

                let mut level3file = brotli::CompressorWriter::new(
                    File::create(
                        output_dir.to_owned()
                            + "/"
                            + (index_main).to_string().as_str()
                            + "-"
                            + (index).to_string().as_str()
                            + ".l3.raw",
                    )
                    .expect("create failed"),
                    40960,
                    11,
                    21,
                );

                let part_center = get_triangle_center(&t, sphere_radius);

                let mut level1 = subdivide_triangle_multiple(t, subdivide_level1);
                let mut level2 = subdivide_triangle_multiple(t, subdivide_level2);
                let mut level3 = subdivide_triangle_multiple(t, subdivide_level3);

                let global_index = (index_main * level0_len) as u32 + index as u32;

                level1.iter_mut().for_each(|t| {
                    let t = normalize_triangle(&t);
                    let vec0dir = t[0].clone().normalize();
                    let vec1dir = t[1].clone().normalize();
                    let vec2dir = t[2].clone().normalize();
                    let directions_triangle: Triangle = [vec0dir, vec1dir, vec2dir];
                    let t = scale_triangle(&t, height_data);
                    let t = translate_triangle(&t, -part_center);
                    match biome_data {
                        None => write_triangle_water(&mut level1file, &t, global_index),
                        Some(biome_data) => write_triangle_terrain(
                            &height_data,
                            &biome_data,
                            &mut level1file,
                            &t,
                            &directions_triangle,
                            global_index,
                        ),
                    }
                });

                level2.iter_mut().for_each(|t| {
                    let t = normalize_triangle(&t);
                    let vec0dir = t[0].clone().normalize();
                    let vec1dir = t[1].clone().normalize();
                    let vec2dir = t[2].clone().normalize();
                    let directions_triangle: Triangle = [vec0dir, vec1dir, vec2dir];
                    let t = scale_triangle(&t, height_data);
                    let t = translate_triangle(&t, -part_center);
                    match biome_data {
                        None => write_triangle_water(&mut level2file, &t, global_index),
                        Some(biome_data) => write_triangle_terrain(
                            &height_data,
                            &biome_data,
                            &mut level2file,
                            &t,
                            &directions_triangle,
                            global_index,
                        ),
                    }
                });

                level3.iter_mut().for_each(|t| {
                    let t = normalize_triangle(&t);
                    let vec0dir = t[0].clone().normalize();
                    let vec1dir = t[1].clone().normalize();
                    let vec2dir = t[2].clone().normalize();
                    let directions_triangle: Triangle = [vec0dir, vec1dir, vec2dir];
                    let t = scale_triangle(&t, height_data);
                    let t = translate_triangle(&t, -part_center);
                    // println!("DIST {}", (t[0] - t[1]).length());
                    match biome_data {
                        None => write_triangle_water(&mut level3file, &t, global_index),
                        Some(biome_data) => write_triangle_terrain(
                            &height_data,
                            &biome_data,
                            &mut level3file,
                            &t,
                            &directions_triangle,
                            global_index,
                        ),
                    }
                });
                level1file.flush().unwrap();
                level2file.flush().unwrap();
                level3file.flush().unwrap();
            });
        });
    metadata_file.flush().unwrap();
}

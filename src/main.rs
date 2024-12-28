mod base_icosphere;
mod cli_args;
mod cubemap_data;
mod erosion;
mod generate_icosphere;
mod json_input;
mod math_util;
mod noise;
mod random;

use crate::cli_args::CLIArgs;
use crate::cubemap_data::{CubeMapDataLayer, CubeMapFace};
use crate::erosion::erosion_run;
use crate::generate_icosphere::generate_icosphere_raw;
use crate::json_input::parse_input_data;
use crate::noise::fbm;
use crate::random::random_1d_to_array;
use clap::Parser;
use glam::{DVec2, DVec3};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::f64::consts::PI;
use std::fs;
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::time::Instant;

fn polar_to_xyz(xyin: DVec2) -> DVec3 {
    let xy = xyin * DVec2::new(2.0 * PI, PI);
    let z = xy.y.cos();
    let x = xy.x.cos() * xy.y.sin();
    let y = xy.x.sin() * xy.y.sin();
    DVec3::new(x, y, z).normalize()
}

fn main() {
    let cli_args = CLIArgs::parse();
    let input_json = fs::read_to_string(cli_args.input).expect("Failed to to read the input file");
    let input = parse_input_data(&*input_json);

    let faces = [
        CubeMapFace::PX,
        CubeMapFace::PY,
        CubeMapFace::PZ,
        CubeMapFace::NX,
        CubeMapFace::NY,
        CubeMapFace::NZ,
    ];

    let cube_map_res = input.generator_config.cube_map_resolution;
    let mut cube_map: CubeMapDataLayer = CubeMapDataLayer::new(cube_map_res);

    let start = Instant::now();

    let mutablefaces = [
        (CubeMapFace::PX, cube_map.get_mutable_face(&CubeMapFace::PX)),
        (CubeMapFace::PY, cube_map.get_mutable_face(&CubeMapFace::PY)),
        (CubeMapFace::PZ, cube_map.get_mutable_face(&CubeMapFace::PZ)),
        (CubeMapFace::NX, cube_map.get_mutable_face(&CubeMapFace::NX)),
        (CubeMapFace::NY, cube_map.get_mutable_face(&CubeMapFace::NY)),
        (CubeMapFace::NZ, cube_map.get_mutable_face(&CubeMapFace::NZ)),
    ];

    mutablefaces.into_par_iter().for_each(|face| {
        println!("Generating face {}, res: {}", face.0, cube_map_res);
        let mut face_data = face.1.lock().unwrap();
        // (0..cube_map_res).into_iter().for_each(|y| {
        //     (0..cube_map_res).into_iter().for_each(|x| {
        for y in (0..cube_map_res) {
            for x in (0..cube_map_res) {
                let dir = cube_map.pixel_coords_to_direction(&face.0, x as usize, y as usize);
                let value = if args.fbm_iterations == 0 {
                    0.0
                } else {
                    fbm(
                        dir * args.fbm_scale,
                        args.fbm_iterations,
                        args.fbm_iteration_scale_coef,
                        args.fbm_iteration_weight_coef,
                    )
                };
                let index = (y as usize) * (cube_map_res as usize) + (x as usize);
                face_data[index] =
                    args.radius + args.terrain_height * value.powf(args.fbm_final_pow);
            }
        }
    });

    if args.erosion_iterations > 0 {
        erosion_run(
            &mut cube_map,
            args.erosion_iterations,
            args.erosion_droplets_count,
            args.radius,
        );
    }

    faces.clone().into_par_iter().for_each(|face| {
        println!("Saving height face {}, res: {}", face, cube_map_res);
        let mut imgbuf = image::ImageBuffer::new(cube_map_res as u32, cube_map_res as u32);
        imgbuf.par_enumerate_pixels_mut().for_each(|(x, y, pixel)| {
            let dir = cube_map.pixel_coords_to_direction(&face, x as usize, y as usize);
            let value = cube_map.get(dir);

            *pixel = image::Luma([(value * 255.0) as u8]);
        });
        imgbuf
            .save(format!("{}/height_face_{}.png", args.out_dir, face))
            .unwrap();
        imgbuf
            .save(format!("cubemap_visualizer/public/face_{}.png", face))
            .unwrap();
    });

    faces.clone().into_par_iter().for_each(|face| {
        println!("Saving normal face {}, res: {}", face, cube_map_res);
        let mut imgbuf = image::ImageBuffer::new(cube_map_res as u32, cube_map_res as u32);
        imgbuf.par_enumerate_pixels_mut().for_each(|(x, y, pixel)| {
            let dir = cube_map.pixel_coords_to_direction(&face, x as usize, y as usize);
            let value = cube_map.get_normal(dir, cube_map.get_pixel_distance_for_dir(dir));

            *pixel = image::Rgb([
                (value.x * 255.0) as u8,
                (value.y * 255.0) as u8,
                (value.z * 255.0) as u8,
            ]);
        });
        imgbuf
            .save(format!("{}/normal_face_{}.png", args.out_dir, face))
            .unwrap();
        imgbuf
            .save(format!(
                "cubemap_visualizer/public/normal_face_{}.png",
                face
            ))
            .unwrap();
    });
    //
    // println!("Saving icosphere");
    // generate_icosphere_raw(
    //     args.out_dir.as_str(),
    //     &cube_map,
    //     args.radius,
    //     args.subdivide_initial,
    //     args.subdivide_level1,
    //     args.subdivide_level2,
    //     args.subdivide_level3,
    // );

    let duration = start.elapsed();
    println!("Generation finished in: {:?}", duration);
}

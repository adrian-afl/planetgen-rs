use crate::cubemap_data::CubeMapDataLayer;
use crate::generate_terrain::InterpolatedBiomeData;
use crate::math_util::map;
use crate::random::{random_1d_to_1d, random_1d_to_3d};
use glam::{DQuat, DVec3};
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelIterator;
use std::f64::consts::PI;

pub fn add_craters(
    cube_map_height: &mut CubeMapDataLayer<f64>,
    cube_map_biome: &CubeMapDataLayer<InterpolatedBiomeData>,
    terrain_radius: f64,
    seed: f64,
    count: u32,
) {
    println!("Adding craters");
    let mut added = 0;
    let mut seed = seed;
    loop {
        let random_dir = (random_1d_to_3d(seed) * 2.0 - 1.0).normalize();
        let biome = cube_map_biome.get(random_dir);

        let size = map(
            random_1d_to_1d(seed).powf(2.0), // make smaller craters more likely
            0.0,
            1.0,
            biome.min_crater_size as f64,
            biome.max_crater_size as f64,
        );
        let probability = random_1d_to_1d(seed + 135.123);

        seed += 567567.45546567;

        if probability < biome.craters_probability as f64 {
            continue;
        }

        let pixel_size = cube_map_height.get_smallest_pixel_distance();
        let distance_steps = (size / (pixel_size * terrain_radius)) as i32 * 20;

        let surface_tangent = if random_dir.y.abs() < 0.99 {
            DVec3::new(0.0, 1.0, 0.0).cross(random_dir).normalize()
        } else {
            DVec3::new(1.0, 0.0, 0.0).cross(random_dir).normalize()
        };

        (0..distance_steps).into_par_iter().for_each(|d_step| {
            let d = (d_step as f64) * pixel_size;
            let percentage = d_step as f64 / distance_steps as f64;

            let depth = if percentage > 0.6 {
                // looking from edge to center
                1.0 - ((percentage - 0.6) / 0.4) // this needs to go from 0 to 1
            } else {
                ((percentage / 0.6 - 1.0) * 3.0) + 1.0 // this needs to go from 1 to like -3 or -4
            };
            let depth = depth * 0.00003 * size;

            let dist = (d_step as f64 / distance_steps as f64) * (size / terrain_radius);

            let ciricumfence = 2.0 * PI * dist;
            let circle_steps = (ciricumfence / pixel_size) as i32 * 100 + 50;
            for a_step in 0..circle_steps {
                let rad = 2.0 * PI * (a_step as f64 / circle_steps as f64);
                let orient = DQuat::from_axis_angle(random_dir, rad);
                let vec = (random_dir + orient * surface_tangent * dist).normalize();
                cube_map_height.add(vec, depth);
            }
        });

        added += 1;

        println!("{added} / {count}");

        if added >= count {
            break;
        }
    }
}

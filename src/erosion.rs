/*
EROSION IDEA to make it parallel

Create an ErosionDropletRun struct
it will hold a vector list of struct of shape ErosionDropletModification {
    vec3 dir (maybe pixels im not sure) pixels will be easier to apply
    float delta
}

then in paraller per iteration i can spawn a droplet on random X pixels - maybe on all of them?
iterate all teh droplers in parallel until all end their journeys
resulting changes are then applied on the main data and the iteration restarts
 */
use crate::cubemap_data::CubeMapDataLayer;
use crate::random::random_2d_to_3d;
use glam::{DVec2, DVec3};
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelIterator;
use std::sync::{Arc, Mutex};

struct ErosionDroplet {
    position: DVec3,
    velocity: DVec3,
    accumulation: f64,
    water_left: f64,
}

struct ErosionDropletModification {
    position: DVec3,
    delta: f64,
}

fn asymptotic(x: f64, limit: f64) -> f64 {
    (1.0 - 1.0 / x.exp()) * limit
}

fn get_surface_normal(cubemap_data: &CubeMapDataLayer, smooth_normal: DVec3) -> DVec3 {
    cubemap_data.get_normal(
        smooth_normal,
        cubemap_data.get_pixel_distance_for_dir(smooth_normal),
    )
}

fn update_droplet_velocity(
    droplet: &mut ErosionDroplet,
    smooth_normal: DVec3,
    surface_normal: DVec3,
) {
    let surface_velocity_vector = surface_normal - smooth_normal;

    let velocity_surface_vector = surface_velocity_vector * 100.0;

    droplet.velocity = droplet.velocity.lerp(velocity_surface_vector, 0.15);
}

fn update_droplet_position(droplet: &mut ErosionDroplet, sphere_radius: f64) {
    droplet.position += droplet.velocity * 100.0;
    droplet.position = sphere_radius * droplet.position.normalize();
}

fn evaporate_droplet(droplet: &mut ErosionDroplet) {
    droplet.water_left -= 0.001;
}

fn get_droplet_erosion(droplet: &mut ErosionDroplet, slope: f64) -> f64 {
    let erode: f64 =
        (slope * 0.2 * droplet.water_left.powf(2.0) * droplet.velocity.length().powf(2.0))
            .min(100.0);

    droplet.accumulation += erode;

    erode
}

fn get_droplet_deposit(droplet: &mut ErosionDroplet, slope: f64) -> f64 {
    let deposit =
        0.5 * (1.0 - slope) * (droplet.accumulation) / (droplet.velocity.length() + 1.0).powf(2.0);
    droplet.accumulation = (droplet.accumulation - deposit).max(0.0);

    deposit
}

pub fn erosion_run(
    cubemap_data: &mut CubeMapDataLayer,
    iterations: u16,
    droplets_per_iteration: u16,
    sphere_radius: f64,
) {
    let finished_iters = Arc::new(Mutex::from(0_i32));
    (0..iterations).into_par_iter().for_each(|iteration| {
        for droplet_num in (0..droplets_per_iteration) {
            let mut droplet = ErosionDroplet {
                position: sphere_radius
                    * (random_2d_to_3d(DVec2::new(iteration as f64, droplet_num as f64)) * 2.0
                        - 1.0)
                        .normalize(),
                velocity: DVec3::new(0.0, 0.0, 0.0),
                accumulation: 0.0,
                water_left: 1.0,
            };

            while droplet.water_left > 0.0 {
                let smooth_normal = droplet.position.clone().normalize();
                let surface_normal = get_surface_normal(&cubemap_data, smooth_normal);

                let slope = 1.0 - surface_normal.dot(smooth_normal).max(0.0).powf(4.0);

                update_droplet_velocity(&mut droplet, smooth_normal, surface_normal);

                let mut delta = 0.0;

                delta -= get_droplet_erosion(&mut droplet, slope);
                delta += get_droplet_deposit(&mut droplet, slope);

                cubemap_data.add(smooth_normal, delta);

                update_droplet_position(&mut droplet, sphere_radius);

                evaporate_droplet(&mut droplet);

                if (droplet.velocity.length() < 0.001) {
                    cubemap_data.add(smooth_normal, droplet.accumulation);
                    break;
                }
            }
        }
        let mut finished_iters = finished_iters.lock().unwrap();
        *finished_iters += 1;
        println!("Erosion iteration: {}/{iterations}", finished_iters);
    })
}

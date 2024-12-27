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

struct ErosionDropletRunResult {
    modifications: Vec<ErosionDropletModification>,
}

fn asymptotic(x: f64, limit: f64) -> f64 {
    (1.0 - 1.0 / x.exp()) * limit
}

pub fn erosion_run(
    cubemap_data: &mut CubeMapDataLayer,
    iterations: u16,
    droplets_per_iteration: u16,
    sphere_radius: f64,
    terrain_height: f64,
) {
    (0..iterations).for_each(|iteration| {
        println!("Erosion iteration: {iteration}");
        let run_results: Vec<ErosionDropletRunResult> = (0..droplets_per_iteration)
            .into_par_iter()
            .map(|droplet_num| {
                let mut run = ErosionDropletRunResult {
                    modifications: Vec::new(),
                };
                let mut droplet = ErosionDroplet {
                    position: (sphere_radius)
                        * (random_2d_to_3d(DVec2::new(iteration as f64, droplet_num as f64)) * 2.0
                            - 1.0)
                            .normalize(),
                    velocity: DVec3::new(0.0, 0.0, 0.0),
                    accumulation: 0.0,
                    water_left: 1.0,
                };
                while droplet.water_left > 0.0 {
                    let mut delta = 0.0;

                    let erode_deposit_coef = (1.0
                        / (droplet.velocity.length() * droplet.velocity.length() * 1000.0 + 1.0));

                    let erode: f64 = (1.0 - erode_deposit_coef) * 0.000004;
                    delta -= erode;

                    // droplet.accumulation += erode;
                    //
                    // let deposit = (droplet.accumulation * erode_deposit_coef * 0.1).min(0.0001);
                    // droplet.accumulation -= deposit;
                    // droplet.accumulation = droplet.accumulation.max(0.0);
                    // delta += deposit;

                    run.modifications.push(ErosionDropletModification {
                        position: droplet.position.clone().normalize(),
                        delta,
                    });

                    let smooth_normal = droplet.position.clone().normalize();
                    let real_normal = cubemap_data.get_normal(
                        droplet.position.clone().normalize(),
                        0.01,
                        sphere_radius,
                        terrain_height,
                    );
                    let surface_velocity_vector = real_normal - smooth_normal;

                    let velocity_surface_vector = surface_velocity_vector * 100.0;

                    droplet.velocity = droplet.velocity.lerp(velocity_surface_vector, 0.015);

                    if (droplet.velocity.length() < 0.001) {
                        break;
                    }

                    // water droplet terminal velocity is 9 m/2 so lets limit the velocity to this
                    // droplet.velocity = droplet.velocity.clone().normalize()
                    //     * asymptotic(droplet.velocity.length() * 0.1, 9.0);

                    droplet.position += droplet.velocity * 200.0;
                    droplet.position = sphere_radius * droplet.position.clone().normalize();

                    droplet.water_left -= 0.001;

                    // let mut deposit =
                    //     droplet.accumulation * (10.0 - droplet.velocity.length()).max(0.0) * 10.1;
                    // run.modifications.push(ErosionDropletModification {
                    //     position: droplet.position.clone().normalize(),
                    //     delta: deposit,
                    // });
                    // droplet.accumulation -= deposit;
                    // droplet.accumulation = droplet.accumulation.max(0.0);

                    // if (droplet_num == 0 && iteration == 0) {
                    //     println!("erode_deposit_coef {erode_deposit_coef}",)
                    // }
                }
                // run.modifications.push(ErosionDropletModification {
                //     position: droplet.position.clone().normalize(),
                //     delta: droplet.accumulation,
                // });
                run
            })
            .collect();

        run_results.iter().for_each(|result| {
            result.modifications.iter().for_each(|modification| {
                cubemap_data.add(modification.position, modification.delta);
            })
        })
    })
}

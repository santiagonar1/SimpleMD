use simple_md::{particle::Particle, simulation::Simulation};

fn main() {
    let pos1 = [1.0, 2.0, 3.0];
    let v1 = [1.0, 2.0, 3.0];
    let f1 = [1.0, 2.0, 3.0];
    let m1 = 6.0;
    let p1 = Particle::new(m1, pos1, v1, f1);

    let pos2 = [1.0, 0.0, 5.0];
    let v2 = [3.0, 2.0, 2.0];
    let f2 = [4.0, 2.0, 3.0];
    let m2 = 6.0;
    let p2 = Particle::new(m2, pos2, v2, f2);

    let delta_t = 1.0;
    let t_end = 10.0;
    let mut simulation = Simulation::new(vec![p1, p2], delta_t, t_end);
    simulation.run();

    println!("Done!");
}

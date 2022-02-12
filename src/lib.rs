pub mod definitions;

#[derive(Clone)]
struct Particle {
    m: definitions::Real,
    pos: [definitions::Real; 3],
    v: [definitions::Real; 3],
    f: [definitions::Real; 3],
    f_old: [definitions::Real; 3],
}

struct Simulation {
    particles: Vec<Particle>,
    delta_t: definitions::Real,
    t_end: definitions::Real,
}

impl Particle {
    fn new(
        m: definitions::Real,
        pos: [definitions::Real; 3],
        v: [definitions::Real; 3],
        f: [definitions::Real; 3],
    ) -> Particle {
        Particle {
            m,
            pos,
            v,
            f,
            f_old: f,
        }
    }

    fn add_f_from(&mut self, other: &Particle) -> Vec<definitions::Real> {
        let mut f_from_other = Vec::with_capacity(3);
        let r: definitions::Real = self
            .pos
            .iter()
            .zip(other.pos.iter())
            .map(|(x1, x2)| (x1 - x2).powi(2))
            .sum();
        let f = (self.m * other.m) / (r.sqrt() * r);
        for d in 0..self.pos.len() {
            f_from_other.push(f * (other.pos[d] - self.pos[d]));
            self.f[d] += f_from_other[d];
        }

        f_from_other
    }
}

fn include_forces(p1: &mut Particle, p2: &mut Particle) {
    let f_p2_into_p1 = p1.add_f_from(&p2);
    for d in 0..p1.pos.len() {
        p2.f[d] -= f_p2_into_p1[d];
    }
}

impl Simulation {
    fn update_pos(&mut self) {
        for particle in &mut self.particles {
            let a = self.delta_t * (0.5 / particle.m);
            for d in 0..particle.pos.len() {
                particle.pos[d] += self.delta_t * (particle.v[d] + a * particle.f[d]);
                particle.f_old[d] = particle.f[d];
            }
        }
    }

    fn update_v(&mut self) {
        for particle in &mut self.particles {
            let a = self.delta_t * (0.5 / particle.m);
            for d in 0..particle.v.len() {
                particle.v[d] += a * (particle.f[d] + particle.f_old[d]);
            }
        }
    }

    fn update_f(&mut self) {
        let l = self.particles.len();
        for i in 0..l {
            let mut particle = self.particles[i].clone();
            for j in (i + 1)..l {
                include_forces(&mut particle, &mut self.particles[j]);
            }
            self.particles[i] = particle;
        }
    }

    fn run(&mut self) {
        let mut t = 0.0;

        self.update_f();
        while t < self.t_end {
            self.update_pos();
            self.update_f();
            self.update_v();
            t += self.delta_t;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_particle() {
        let particle = Particle::new(6.0, [1.0, 2.0, 3.0], [1.0, 2.0, 3.0], [1.0, 2.0, 3.0]);

        assert_eq!(particle.m, 6.0);
        assert_eq!(particle.pos, [1.0, 2.0, 3.0]);
        assert_eq!(particle.v, [1.0, 2.0, 3.0]);
        assert_eq!(particle.f, [1.0, 2.0, 3.0]);
    }

    #[test]
    fn create_simulation() {
        let pos = [1.0, 2.0, 3.0];
        let v = [1.0, 2.0, 3.0];
        let f = [1.0, 2.0, 3.0];
        let _simulation = Simulation {
            particles: vec![Particle::new(6.0, pos, v, f)],
            delta_t: 1.0,
            t_end: 10.0,
        };
    }

    #[test]
    fn update_position() {
        let pos = [1.0, 2.0, 3.0];
        let v = [1.0, 2.0, 3.0];
        let f = [1.0, 2.0, 3.0];
        let delta_t = 1.0;
        let t_end = 10.0;
        let m = 6.0;
        let mut simulation = Simulation {
            particles: vec![Particle::new(m, pos, v, f)],
            delta_t,
            t_end,
        };

        simulation.update_pos();

        let mut expected_new_pos = [0.0, 0.0, 0.0];
        let a = delta_t * 0.5 / m;
        for d in 0..expected_new_pos.len() {
            expected_new_pos[d] = pos[d] + delta_t * (v[d] + a * f[d]);
        }
        assert_eq!(simulation.particles[0].pos, expected_new_pos);
    }

    #[test]
    fn update_velocity() {
        let pos = [1.0, 2.0, 3.0];
        let v = [1.0, 2.0, 3.0];
        let f = [1.0, 2.0, 3.0];
        let delta_t = 1.0;
        let t_end = 10.0;
        let m = 6.0;
        let mut simulation = Simulation {
            particles: vec![Particle::new(m, pos, v, f)],
            delta_t,
            t_end,
        };

        simulation.update_v();

        let mut expected_new_v = [0.0, 0.0, 0.0];
        let a = delta_t * 0.5 / m;
        for d in 0..expected_new_v.len() {
            expected_new_v[d] = v[d] + a * (f[d] + f[d]);
        }
        assert_eq!(simulation.particles[0].v, expected_new_v);
    }

    #[test]
    fn update_f() {
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
        let mut simulation = Simulation {
            particles: vec![p1, p2],
            delta_t,
            t_end,
        };

        simulation.update_f();

        let p1 = Particle::new(m1, pos1, v1, f1);
        let p2 = Particle::new(m2, pos2, v2, f2);
        let mut expected_new_f_p1 = [0.0, 0.0, 0.0];
        let mut expected_new_f_p2 = [0.0, 0.0, 0.0];
        let r: definitions::Real = p1
            .pos
            .iter()
            .zip(p2.pos.iter())
            .map(|(x1, x2)| (x2 - x1).powi(2))
            .sum();
        let f = (p1.m * p2.m) / (r.sqrt() * r);
        for d in 0..expected_new_f_p1.len() {
            expected_new_f_p1[d] = p1.f[d] + f * (p2.pos[d] - p1.pos[d]);
            expected_new_f_p2[d] = p2.f[d] - f * (p2.pos[d] - p1.pos[d]);
        }

        assert_eq!(simulation.particles[0].f, expected_new_f_p1);
        assert_eq!(simulation.particles[1].f, expected_new_f_p2);
    }
}

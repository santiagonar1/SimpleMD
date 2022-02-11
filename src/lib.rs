type Real = f64;

struct Particle {
    m: Real,
    pos: [Real; 3],
    v: [Real; 3],
    f: [Real; 3],
    f_old: [Real; 3],
}

struct Simulation {
    particles: Vec<Particle>,
    delta_t: Real,
}

impl Particle {
    fn new(m: Real, pos: [Real; 3], v: [Real; 3], f: [Real; 3]) -> Particle {
        Particle {
            m,
            pos,
            v,
            f,
            f_old: f,
        }
    }

    fn add_f_from(&mut self, other: &Particle) -> Vec<Real> {
        let mut f_from_other = Vec::with_capacity(3);
        let r: Real = self
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
        };
    }

    #[test]
    fn update_position() {
        let pos = [1.0, 2.0, 3.0];
        let v = [1.0, 2.0, 3.0];
        let f = [1.0, 2.0, 3.0];
        let delta_t = 1.0;
        let m = 6.0;
        let mut simulation = Simulation {
            particles: vec![Particle::new(m, pos, v, f)],
            delta_t,
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
        let m = 6.0;
        let mut simulation = Simulation {
            particles: vec![Particle::new(m, pos, v, f)],
            delta_t,
        };

        simulation.update_v();

        let mut expected_new_v = [0.0, 0.0, 0.0];
        let a = delta_t * 0.5 / m;
        for d in 0..expected_new_v.len() {
            expected_new_v[d] = v[d] + a * (f[d] + f[d]);
        }
        assert_eq!(simulation.particles[0].v, expected_new_v);
    }
}

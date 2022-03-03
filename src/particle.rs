use crate::definitions::Real;

#[derive(Clone)]
pub struct Particle<const D: usize> {
    pub m: Real,
    pub pos: [Real; D],
    pub v: [Real; D],
    pub f: [Real; D],
    pub f_old: [Real; D],
}

impl<const D: usize> Particle<D> {
    pub fn new(m: Real, pos: [Real; D], v: [Real; D], f: [Real; D]) -> Particle<D> {
        Particle {
            m,
            pos,
            v,
            f,
            f_old: f,
        }
    }

    pub fn add_f_from(&mut self, other: &Particle<D>) -> Vec<Real> {
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

    pub fn update_pos(&mut self, delta_t: Real) {
        let a = delta_t * (0.5 / self.m);
        self.pos
            .iter_mut()
            .enumerate()
            .for_each(|(d, p)| *p += delta_t * (self.v[d] + a * self.f[d]));
    }

    pub fn update_v(&mut self, delta_t: Real) {
        let a = delta_t * (0.5 / self.m);
        self.v
            .iter_mut()
            .enumerate()
            .for_each(|(d, v)| *v += a * (self.f[d] + self.f_old[d]));
    }

    pub fn update_old_f(&mut self) {
        self.f_old.copy_from_slice(&self.f);
    }
}

pub fn include_forces<const D: usize>(p1: &mut Particle<D>, p2: &mut Particle<D>) {
    let f_p2_into_p1 = p1.add_f_from(p2);
    for (d, force) in f_p2_into_p1.iter().enumerate() {
        p2.f[d] -= force;
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
    fn update_position() {
        let pos = [1.0, 2.0, 3.0];
        let v = [1.0, 2.0, 3.0];
        let f = [1.0, 2.0, 3.0];
        let delta_t = 1.0;
        let m = 6.0;
        let mut particle = Particle::new(m, pos, v, f);

        particle.update_pos(delta_t);

        let mut expected_new_pos = [0.0, 0.0, 0.0];
        let a = delta_t * 0.5 / m;
        for d in 0..expected_new_pos.len() {
            expected_new_pos[d] = pos[d] + delta_t * (v[d] + a * f[d]);
        }
        assert_eq!(particle.pos, expected_new_pos);
    }

    #[test]
    fn update_velocity() {
        let pos = [1.0, 2.0, 3.0];
        let v = [1.0, 2.0, 3.0];
        let f = [1.0, 2.0, 3.0];
        let delta_t = 1.0;
        let m = 6.0;
        let mut particle = Particle::new(m, pos, v, f);

        particle.update_v(delta_t);

        let mut expected_new_v = [0.0, 0.0, 0.0];
        let a = delta_t * 0.5 / m;
        for d in 0..expected_new_v.len() {
            expected_new_v[d] = v[d] + a * (f[d] + f[d]);
        }
        assert_eq!(particle.v, expected_new_v);
    }

    #[test]
    fn update_old_force() {
        let pos = [1.0, 2.0, 3.0];
        let v = [1.0, 2.0, 3.0];
        let f = [1.0, 2.0, 3.0];
        let m = 6.0;
        let mut particle = Particle::new(m, pos, v, f);

        let expected_f = [2.0, 3.0, 4.0];
        particle.f = expected_f;

        particle.update_old_f();

        assert_eq!(particle.f_old, expected_f);
    }
}

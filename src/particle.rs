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
}

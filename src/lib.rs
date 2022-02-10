type Real = f64;

struct Atom {
    m: Real,
    pos: [Real; 3],
    v: [Real; 3],
    f: [Real; 3],
}

struct Simulation {
    atoms: Vec<Atom>,
    delta_t: Real,
}

impl Simulation {
    fn update_pos(&mut self) {
        for atom in &mut self.atoms {
            let a = self.delta_t * (0.5 / atom.m);
            for d in 0..atom.pos.len() {
                atom.pos[d] += self.delta_t * (atom.v[d] + a * atom.f[d]);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_atom() {
        let atom = Atom {
            m: 6.0,
            pos: [1.0, 2.0, 3.0],
            v: [1.0, 2.0, 3.0],
            f: [1.0, 2.0, 3.0],
        };

        assert_eq!(atom.m, 6.0);
        assert_eq!(atom.pos, [1.0, 2.0, 3.0]);
        assert_eq!(atom.v, [1.0, 2.0, 3.0]);
        assert_eq!(atom.f, [1.0, 2.0, 3.0]);
    }

    #[test]
    fn create_simulation() {
        let pos = [1.0, 2.0, 3.0];
        let v = [1.0, 2.0, 3.0];
        let f = [1.0, 2.0, 3.0];
        let _simulation = Simulation {
            atoms: vec![Atom {
                m: 6.0,
                pos: pos,
                v: v,
                f: f,
            }],
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
            atoms: vec![Atom {
                m,
                pos: pos,
                v: v,
                f: f,
            }],
            delta_t,
        };

        simulation.update_pos();

        let mut expected_new_pos = [0.0, 0.0, 0.0];
        let a = delta_t * 0.5 / m;
        for d in 0..expected_new_pos.len() {
            expected_new_pos[d] = pos[d] + delta_t * (v[d] + a * f[d]);
        }
        assert_eq!(simulation.atoms[0].pos, expected_new_pos);
    }
}

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
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

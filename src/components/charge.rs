use crate::components::point;

pub const MASS: f64 = 1.0;
pub const K: f64 = 1.0;
pub const PLANCK: f64 = 1.616e-35;
pub const C: f64 = 3.0e8;

#[derive(Debug, Copy, Clone)]
pub struct Charge{
    pos: point::Point,
    v: point::Point,
    past_a: point::Point,
    q: f64,
    fixed: bool,
}

impl Charge {
    pub fn new(pos: point::Point, v: point::Point, q: f64, fixed: bool)  -> Charge { Charge { pos: pos, v: v, past_a: point::Point::new(), q: q, fixed: fixed} }

    pub fn update(&mut self, f: point::Point, dt: f64) {
        let acc = point::Point::scalar_divide(MASS,f);

        //println!("Acc {}", acc);

        let l = point::Point::scalar_times(dt,self.v);
        let r = point::Point::scalar_times(0.5*dt*dt, acc);
        //println!("Old {}", self.pos);
        self.pos = point::Point::add(self.pos, point::Point::add(l,r));
        //println!("New {}", self.pos);
        self.v = point::Point::add(self.v, point::Point::scalar_times(dt, acc));
        //println!("V {}", self.v);
        self.past_a = acc;
    }

    pub fn get_magnitude(self) -> f64 { self.q }

    pub fn lorentz(self, e: impl Fn(point::Point) -> point::Point, b: impl Fn(point::Point) -> point::Point) -> point::Point {
        return point::Point::scalar_times(self.q ,point::Point::add(e(self.pos), point::Point::cross(self.v, b(self.pos))))
    }

    pub fn abraham_lorentz(c: Charge, f: point::Point, dt: f64) -> point::Point {
        let acc = point::Point::scalar_divide(MASS, f);
        let jerk = point::Point::scalar_divide(dt, point::Point::add(acc, c.past_a.neg()));
        let constants = (2.0/3.0)*K*c.q*c.q/(C*C*C);
        let mut ab_lor_force = point::Point::scalar_times(constants, jerk);
        ab_lor_force = ab_lor_force.align(c.v).neg();
        return ab_lor_force;
    }

    pub fn coulomb(c1: Charge, c2: Vec<Charge>) -> point::Point {
        let magnitude = c1.q*K;
        let mut s = point::Point::new();
        for i in c2 {
            let mut r2 = i.pos.clone();
            r2 = r2.neg();
            let mut num = point::Point::add(c1.pos, r2);
            let denom = point::Point::dot(num, num);
            num = num.unit();
    
            s = point::Point::add(s, point::Point::scalar_times(i.q, point::Point::scalar_divide(denom,num)));
        }
    
        return point::Point::scalar_times(magnitude, s);
    }

    pub fn display_pos(self) -> String {
        return self.pos.to_string();
    }

    pub fn get_pos(self) -> point::Point {
        return self.pos;
    }

    pub fn alter_pos(&mut self, x: f64, y: f64, z: f64) {
        self.pos.set_pos(x, y, z);
    }

    pub fn x(self) -> f64{
        return self.pos.x();
    }

    pub fn y(self) -> f64{
        return self.pos.y();
    }

    pub fn z(self) -> f64{
        return self.pos.z();
    }

    pub fn is_fixed(self) -> bool { self.fixed }
}
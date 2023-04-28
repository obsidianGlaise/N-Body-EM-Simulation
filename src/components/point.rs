use std::fmt;

#[derive(Debug, Copy, Clone)]
pub struct Point(f64,f64,f64);

impl Point {
    pub fn new() -> Point { Point(0.0,0.0,0.0) }

    pub fn from(x: f64, y: f64, z: f64) -> Point {
        return Point(x,y,z);
    }
    
    pub fn add(a: Point, b: Point) -> Point {
        return Point(a.0+b.0,a.1+b.1,a.2+b.2);
    }

    pub fn scalar_times(s: f64, v: Point) -> Point {
        return Point(s*v.0,s*v.1,s*v.2);
    }
    
    pub fn scalar_divide(s: f64, v: Point) -> Point {
        return Point(v.0/s,v.1/s,v.2/s);
    }

    pub fn dot(a: Point, b: Point) -> f64 {
        return a.0*b.0+a.1*b.1+a.2*b.2;
    }
    
    pub fn cross(a: Point, b: Point) -> Point {
        return Point(a.1*b.2-a.2*b.1,a.2*b.0-a.0*b.2,a.0*b.1-a.1*b.0);
    }

    pub fn unit(self) -> Point {
        if self.mag() == 0.0 {
            return Point(0.0,0.0,0.0);
        }
        Point(self.0/self.mag(), self.1/self.mag(), self.2/self.mag())
    }

    pub fn neg(self) -> Point { Point(-self.0,-self.1,-self.2) }

    pub fn mag(self) -> f64 {
        return f64::powf(f64::powf(self.0,2.0)+f64::powf(self.1,2.0)+f64::powf(self.2,2.0),0.5)
    }

    pub fn set_pos(&mut self, x: f64, y: f64, z: f64) {
        self.0 = x;
        self.1 = y;
        self.2 = z;
    }

    pub fn align(self, aligning_vector: Point) -> Point {
        return Self::scalar_times(self.mag(), aligning_vector.unit());
    }

    pub fn x(self) -> f64 { self.0 }
    pub fn y(self) -> f64 { self.1 }
    pub fn z(self) -> f64 { self.2 }


}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{},{})", self.0,self.1,self.2)
    }
}
use std::{
    fs::File,
    io::{BufWriter, Write},
};

use components::{charge::*,point::*};

mod components;
mod util;

fn simulation(b_field: impl Fn(components::point::Point) -> components::point::Point, e_field: impl Fn(components::point::Point) -> components::point::Point,t_max: f64, particles: &mut Vec<Charge>, writer: &mut BufWriter<&File>) {
    let mut t = 0.0;
    let mut x = 0;
 
    for i in particles.clone() {
        //println!("{}: {} {}", x,i.display_pos(), i.get_magnitude());
        writeln!(writer, "{}: {} {}", x,i.display_pos(), i.get_magnitude()).unwrap();
        x+=1;
    }

    while t < t_max {
        let mut new_forces: Vec<Point> = vec![];

        for i in 0..particles.len() {
            let j = particles[i];
            let mut p = particles.clone();
            p.swap_remove(i);
            let cs = components::charge::Charge::coulomb(j, p);
            new_forces.push(Point::add( components::charge::Charge::lorentz(j,&e_field, &b_field),cs));
            
        }

        for i in 0..particles.len() {
            if !(particles[i].is_fixed()) {
                particles[i].update(new_forces[i]);
                //println!("{}: {}", i,particles[i].display_pos());
                writeln!(writer, "{}: {}", i,particles[i].display_pos()).unwrap();
            }
        }
        for i in 0..particles.len() {
            for j in 0..particles.len() {
                if i != j {
                    let dist = Point::add(particles[i].get_pos(),particles[j].get_pos().neg());
                    if dist.mag() < components::charge::PLANCK && !(particles[j].is_fixed()) && !(particles[i].is_fixed()) {
                        let new_pos: Point = Point::scalar_times(components::charge::PLANCK,dist.unit());
                        particles[j].alter_pos(new_pos.x(), new_pos.y(), new_pos.z());
                        println!("SEPARATED!");
                    }
                }
            }
        }

        t+=components::charge::DT;
    }
}


fn main() {
    println!("STARTED SETUP.");
    let (mut particles, (e_coeffs,e_pows), (b_coeffs,b_pows), max_t) = util::setup();
    println!("FINISHED SETUP.");

    let e_field = |p: components::point::Point| -> components::point::Point {
        components::point::Point::from(
            e_coeffs[0]*f64::powf(p.x(),e_pows[0]),
            e_coeffs[1]*f64::powf(p.y(),e_pows[0]),
            e_coeffs[2]*f64::powf(p.z(),e_pows[0])
        )
    };

    let b_field = |p: Point| -> Point {
        components::point::Point::from(
            b_coeffs[0]*f64::powf(p.x(),b_pows[0]),
            b_coeffs[1]*f64::powf(p.y(),b_pows[0]),
            b_coeffs[2]*f64::powf(p.z(),b_pows[0])
        )
    };
    let write_file = File::create("output.txt").unwrap();
    let mut writer = BufWriter::new(&write_file);
    writeln!(writer, "{} {} {} {} {} {} {} {} {} {} {} {}", e_coeffs[0],e_pows[0],e_coeffs[1],e_pows[1],e_coeffs[2],e_pows[2],
        b_coeffs[0],b_pows[0],b_coeffs[1],b_pows[1],b_coeffs[2],b_pows[2]).unwrap();

    println!("STARTED SIMULATION.");
    simulation(b_field, e_field, max_t, &mut particles,&mut writer);
    println!("FINISHED SIMULATION.");
}

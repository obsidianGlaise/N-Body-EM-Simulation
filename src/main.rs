use std::{
    fs::File,
    io::{BufWriter, Write, BufRead},
    io,
};

use components::{charge::*,point::*};

mod components;
mod util;

fn simulation(b_field: impl Fn(Point) -> Point, e_field: impl Fn(Point) -> Point,t_max: f64, dt: f64, particles: &mut Vec<Charge>, writer: &mut BufWriter<&File>) {
    let mut t = 0.0;
    let mut x = 0;
    let mut n = 1;
    
    let new_write_file = File::create("output/out.csv.0").unwrap();
    let mut new_writer = BufWriter::new(&new_write_file);
    writeln!(new_writer, "xcoord, ycoord, zcoord, scalar").unwrap();
    for i in particles.clone() {
        writeln!(new_writer, "{}, {}, {}, {}", i.x(),i.y(),i.z(),i.get_magnitude()).unwrap();
        writeln!(writer, "{}: {} {}", x,i.display_pos(), i.get_magnitude()).unwrap();
        x+=1;
    }

    while t < t_max {
        let mut new_forces: Vec<Point> = vec![];

        for i in 0..particles.len() {
            let j = particles[i];
            let mut p = particles.clone();
            p.swap_remove(i);
            let cs = Charge::coulomb(j, p);
            let lor = Charge::lorentz(j,&e_field, &b_field);
            let f = Point::add( lor,cs);
            let ab_lor = Charge::abraham_lorentz(j, f, dt);
            new_forces.push(Point::add( ab_lor,f));
        }

        for i in 0..particles.len() {
            if !(particles[i].is_fixed()) {
                particles[i].update(new_forces[i],dt);
                writeln!(writer, "{}: {}", i,particles[i].display_pos()).unwrap();
            }
        }
        for i in 0..particles.len() {
            for j in 0..particles.len() {
                if i != j {
                    let dist = Point::add(particles[i].get_pos(),particles[j].get_pos().neg());
                    if dist.mag() < PLANCK && !(particles[j].is_fixed()) && !(particles[i].is_fixed()) {
                        let new_pos: Point = Point::scalar_times(PLANCK,dist.unit());
                        particles[j].alter_pos(new_pos.x(), new_pos.y(), new_pos.z());
                        println!("SEPARATED!");
                    }
                }
            }
        }

        t+=dt;
        let mut s: String = "output/out.csv.".to_string();
        s.push_str(&n.to_string());
        let new_write_file = File::create(s).unwrap();
        let mut new_writer = BufWriter::new(&new_write_file);
        writeln!(new_writer, "xcoord, ycoord, zcoord, scalar").unwrap();
        for i in particles.clone() {
            writeln!(new_writer, "{}, {}, {}, {}", i.x(),i.y(),i.z(),i.get_magnitude()).unwrap();
        }
        n+=1
    }
}

fn main() {
    println!("STARTED SETUP.");
    //let input = env::args().nth(1).unwrap();
    let mut input = String::new();
    let mut output = String::new();

    println!("Please enter the input file name:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    //let output = env::args().nth(2).unwrap();
    println!("Please enter the output file name:");
    io::stdin()
        .read_line(&mut output)
        .expect("Failed to read input");

    input = input.trim().to_string();
    output = output.trim().to_string();

    let (mut particles, (e_coeffs,e_pows), (b_coeffs,b_pows), max_t, dt) = util::setup(input);
    println!("FINISHED SETUP.");

    let e_field = |p: Point| -> Point {
        Point::from(
            e_coeffs[0]*f64::powf(p.x(),e_pows[0]),
            e_coeffs[1]*f64::powf(p.y(),e_pows[1]),
            e_coeffs[2]*f64::powf(p.z(),e_pows[2])
        )
    };

    let b_field = |p: Point| -> Point {
        Point::from(
            b_coeffs[0]*f64::powf(p.x(),b_pows[0]),
            b_coeffs[1]*f64::powf(p.y(),b_pows[1]),
            b_coeffs[2]*f64::powf(p.z(),b_pows[2])
        )
    };
    let write_file = File::create(output).unwrap();
    let mut writer = BufWriter::new(&write_file);
    writeln!(writer, "{} {} {} {} {} {} {} {} {} {} {} {}", e_coeffs[0],e_pows[0],e_coeffs[1],e_pows[1],e_coeffs[2],e_pows[2],
        b_coeffs[0],b_pows[0],b_coeffs[1],b_pows[1],b_coeffs[2],b_pows[2]).unwrap();

    println!("STARTED SIMULATION.");
    simulation(b_field, e_field, max_t, dt, &mut particles,&mut writer);
    
    println!("FINISHED SIMULATION.");
    println!("PRESS ENTER TO EXIT.");
    let stdin = io::stdin();
    let _ = stdin.lock().lines().next();
}

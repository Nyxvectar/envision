/**
 * Author:  Raye Lattice 
 * Repo:    rustHello
 * Created: 08/24/2025
 */
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();

    match t {
        1 => {
            println!("I love Luogu!");
        }
        2 => {
            let a_and_uim = 2 + 4;
            let b = 10 - a_and_uim;
            println!("{} {}", a_and_uim, b);
        }
        3 => {
            let total = 14;
            let students = 4;
            let per_student = total / students;
            let distributed = per_student * students;
            let remaining = total % students;
            println!("{}", per_student);
            println!("{}", distributed);
            println!("{}", remaining);
        }
        4 => {
            let total = 500.0;
            let students = 3;
            let per_student = total / students as f64;
            println!("{0:.3}", per_student);
        }
        5 => {
            let len1 = 260;
            let speed1 = 12;
            let len2 = 220;
            let speed2 = 20;
            let total_len = len1 + len2;
            let total_speed = speed1 + speed2;
            let time = total_len / total_speed;
            println!("{}", time);
        }
        6 => {
            let length = 6.0;
            let width = 9.0;
            let diagonal = ((length * length + width * width) as f32).sqrt();
            println!("{}", diagonal);
        }
        7 => {
            let mut balance = 100;
            balance += 10;
            println!("{}", balance);
            balance -= 20;
            println!("{}", balance);
            balance = 0;
            println!("{}", balance);
        }
        8 => {
            let r = 5.0;
            let pi = std::f64::consts::PI;
            let circumference = 2.0 * pi * r;
            let area = pi * r * r;
            let volume = (4.0 / 3.0) * pi * r * r * r;
            println!("{}", circumference);
            println!("{}", area);
            println!("{}", volume);
        }
        9 => {
            let mut peaches = 1;
            peaches = (peaches + 1) * 2;
            peaches = (peaches + 1) * 2;
            peaches = (peaches + 1) * 2;
            println!("{}", peaches);
        }
        10 => {
            println!("8");
        }
        11 => {
            let speed_a = 5.0;
            let speed_b = 8.0;
            let distance = 100.0;
            let time = distance / (speed_b - speed_a);
            println!("{}", time);
        }
        12 => {
            println!("13");
            println!("R");
        }
        13 => {
            let r1 = 4.0;
            let r2 = 10.0;
            let pi = std::f32::consts::PI;
            let volume1 = (4.0 / 3.0) * pi * r1 * r1 * r1;
            let volume2 = (4.0 / 3.0) * pi * r2 * r2 * r2;
            let total_volume = volume1 + volume2;
            let edge_length = total_volume.cbrt() as i32;
            println!("{}", edge_length);
        }
        14 => {
            println!("50");
        }
        _ => println!("Invalid problem number"),
    }
}

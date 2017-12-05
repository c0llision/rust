use std::io::{stdin,stdout,Write};

fn calc_area_circle(r: f32) -> f32 {
    let pi: f32 = 3.14;
    let area_circle: f32 = pi * r * r;
    area_circle
}

fn main() {
    let mut s=String::new();

    print!("Enter the radius: ");
    let _=stdout().flush();

    stdin().read_line(&mut s).expect("Did not enter a correct string");
    let radius: f32 = s.trim().parse().unwrap();

    let area_circle: f32 = calc_area_circle(radius);
    println!("Area of circle with radius {} is : {}", radius, area_circle);
}

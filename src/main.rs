use std::io;

fn main() {
    println!("Enter Your Weight (kg):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let weight: f32 = input.trim().parse().unwrap();

    let mut mars_weight = calc_weight_mars(weight);
    println!("weight on mars {}kg", mars_weight);
}

fn calc_weight_mars(weight: f32) -> f32  {
    (weight / 9.81) * 3.711
}
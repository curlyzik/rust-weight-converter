use std::io;

fn main() {
    println!("Enter Your Weight (kg):");
    // make the input mutable
    let mut input = String::new();

    // create a mutable reference to the input
    io::stdin().read_line(&mut input).unwrap();

    // convert the value to float data type
    let weight: f32 = input.trim().parse().unwrap();

    let mars_weight = calc_weight_mars(weight);
    println!("weight on mars {}kg", mars_weight);
}

fn calc_weight_mars(weight: f32) -> f32  {
    (weight / 9.81) * 3.711
}
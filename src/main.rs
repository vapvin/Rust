use std::io;

fn main() {
    println!("Enter ur weight (kg): ");
    let mut input = String::new();    
    io::stdin().read_line(&mut input).unwrap();
    let weight: f32 = input.trim().parse().unwrap();
    println!("Your Weight: {}", weight);
    let mars_weight = calculate_weight_on_mars(weight);
    println!("Your Mars Weight: {}kg", mars_weight);
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    return (weight / 9.81) * 3.711;
}

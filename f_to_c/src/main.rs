use std::io::stdin;

fn main() {
    println!("Please enter the temperature to convert with f or c at the end");
    println!("to indicate Fahrenheit or Celsius.");
    println!("The converter will calculate the corresponding value in the alternate scale");

    let mut temp = String::new();
    stdin().read_line(&mut temp).expect("Could not read your input");

    let digits = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '.'];

    let mut temperature: f32 = 0.0;
    let mut system = String::new();

    for (_i, c) in temp.trim().chars().enumerate() {
        if !digits.contains(&c) {
            let split: Vec<&str> = temp.trim().split(c).collect();
            temperature = split[0].trim().parse().expect("Not a valid temperature");
            system.push_str(split[1].trim());
            break;
        }
    }

    if system.eq_ignore_ascii_case("f") {
        let conv_temp: f32 = to_celsius(temperature);
        println!("{} F is {} C", temperature, conv_temp);
    } 
    else if system.eq_ignore_ascii_case("c") {
        let conv_temp: f32 = to_fahrenheit(temperature);
        println!("{} C is {} F", temperature, conv_temp);
    }
    else {
        println!("Wrong system.  Only Celsias and Fahrenheit are supported");
    }
}

fn to_celsius(_t: f32) -> f32 {
    (_t - 32.0) * 5.0 / 9.0
}

fn to_fahrenheit(_t: f32) -> f32 {
    _t * 9.0 / 5.0 + 32.0
}
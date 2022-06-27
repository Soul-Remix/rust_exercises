use std::io;

fn main() {
   let mut temp = String::new();

   let mut convert_to = String::new();

   println!("Enter the temperature:");

   io::stdin().read_line(&mut temp).expect("failed to read line");

   let temp: f64 = temp.trim().parse().expect("Expected a number");

   println!("Enter what you want to convert to (C or F)");

    io::stdin().read_line(&mut convert_to).expect("failed to read line");

    let convert_to : char = convert_to.to_lowercase().chars().next().expect("string is empty");

    if convert_to != 'c' && convert_to != 'f' {
        println!("Wrong type");
        return
    }

    if convert_to == 'c' {
        let new_temp:f64 = f64::trunc(((temp - 32.0)/1.8)  * 100.0) / 100.0;
        println!("temp is {}C", new_temp);
    } else if convert_to == 'f' {
        let new_temp:f64 = f64::trunc((temp * 1.8 + 32.0)  * 100.0) / 100.0;
        println!("temp is {}F", new_temp);
    }
}
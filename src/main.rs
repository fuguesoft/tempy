// Convert a temperatures from Fahrenheit to Celsius and vice versa
#![allow(unused)]
use std::io;

fn main() {
  // suggested flow
  // Welcome user
  // LOOP START
  // Get unit/value
  // Do Conversion
  // Ask to go again
  // LOOP END
  choose_conversion();
  go_again();
}

fn convert_to_celsius() {
  let value = get_value();
  let conversion = (value - 32) * 5/9;
  println!("{value}ºF is {conversion}ºC");
}

fn convert_to_fahrenheit() {
  let value = get_value();
  let conversion = value * 9/5 + 32;
  println!("{value}ºC is {conversion}ºF");
}

// get conversion unit
fn get_unit() -> String {
  loop {
    let mut unit = String::new();
    println!("choose a unit to convert from: (f)ahrenheit/(c)elsius");
    println!("(quit: Ctrl-C)");

    io::stdin()
      .read_line(&mut unit)
      .expect("Failed to read line");

    let unit = unit.trim();
    println!("unit chosen: {unit}");

    let mut out = String::from(unit);
    return out
  }
}

// get conversion value
fn get_value() -> i32 {
  loop {
    println!("please enter a value to convert");
    let mut value = String::new();

    io::stdin()
      .read_line(&mut value)
      .expect("Failed to read line");

    let value: i32 = match value.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };
    return value
  }
}

fn choose_conversion() {
  loop {
    print!("\x1b[2J\x1b[1;1H");
    println!("welcome to tempy!\na celsius <-> farenheit converter");
    let mut unit = get_unit();
    // println!("ur mum, unit is: {unit}");

    // exo says: do this check inside of get_unit() and 
    // return and enum to match against
    if unit == "c" {
      println!("converting celsius to farenheit");
      convert_to_fahrenheit();
      break
    } else if unit == "f" {
      println!("converting farenheit to celsius");
      convert_to_celsius();
      break
    } else {
      println!("{unit} is not valid.");
      continue
    }
  }
}

fn go_again() {
  loop {
    let mut choice = get_answer();
    if choice == "y" {
      choose_conversion()
    } else if choice == "" {
      choose_conversion()
    } else if choice == "n" {
      return
    } else {
      println!("choice invalid");
      continue
    }
  }
}

fn get_answer() -> String {
  loop {
    println!("Go again? (Y/n)");
    let mut answer = String::new();

    io::stdin()
      .read_line(&mut answer)
      .expect("Failed to read line");

    let out = String::from(answer.trim());
    return out

  }
}

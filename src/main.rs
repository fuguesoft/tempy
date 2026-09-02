// A program to convert a temperatures from Fahrenheit to Celsius and vice versa
#![allow(unused)]
use std::io;

fn main() {
  choose_conversion();
}

fn convert_to_celsius() {
  let value = get_value();
  let conversion = (value - 32) * 5/9;
  println!("{value}ºF is {conversion}ºC");
  return
}

fn convert_to_fahrenheit() {
  let value = get_value();
  let conversion = value * 9/5 + 32;
  println!("{value}ºC is {conversion}ºF");
  return
}

// get conversion unit
fn get_unit() -> String {
  loop {
    let mut unit = String::new();
    println!("choose a unit to convert: (f)ahrenheit/(c)elsius");

    io::stdin()
      .read_line(&mut unit)
      .expect("Failed to read line");

    let unit = unit.trim();
    println!("unit chosen: {unit}");

    let mut out = String::from(unit);
    return out
  }
}

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
    let mut unit = get_unit();
    // println!("ur mum, unit is: {unit}");

    if unit == "c" {
      println!("converting celsius to farenheit");
      convert_to_fahrenheit();
      break
    } else if unit == "f" {
      println!("converting farenheit to celsius");
      convert_to_celsius();
      break
    } else {
      println!("{unit} is not valid, please enter a valid unit:");
      continue
    }
  }
}

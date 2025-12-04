#![allow(unused)]
fn main() {
    // let eight_bit_signed: i16 = -112;
    // let eight_bit_unsigned: u16 = 112;

    // let thirty_two_bit_signed = -2134124123;
    // let thirty_two_bit_unsigned: u32 = 4291231232;

    // let sixteen_bit_signed: i16 = 32_400;

    // let days: usize = 55;
    // let years: isize = -15_000;

    /* println!("Dear Emily, \nHow have you been?");
    println!("\tOnce upon a time");
    println!("Juliet said \"I love you Romeo\"");
    let filepath = r"C:\My Documents\new\videos";
    println!("{filepath}"); */

    /*  let value: i32 = -15;
    println!("{}", value.abs());

    let empty_space = "       my content    ";
    println!("{}", empty_space.trim());

    println!("{}", value.pow(2));
    println!("{}", value.pow(3)); */

    // let pi: f32 = 3.14159265897932384;
    // println!("Current value of pi is {:.2}", pi);
    // println!("{}", pi.floor());
    // println!("{}", pi.ceil());
    // println!("{}", pi.round());

    /* let miles_away = 50;
    let miles_away_i8 = miles_away as i8;
    let miles_away_i8 = miles_away as u8;

    let miles_away = 100.329032;
    let miles_away_32 = miles_away as f32;
    let miles_away_int = miles_away as i32;
    println!("{}", miles_away_int) */

    /* let addition = 5 + 4;
    let subtraction = 5 - 4;
    let multi = 5 * 4;
    println!("{addition}, {subtraction}, {multi}");

    let floor_division = 5 / 3;
    println!("{floor_division}");

    let decimal_division = 5.0 / 3.0;
    println!("{decimal_division}");

    let remainder_operator = 10 % 6;
    println!("{}", remainder_operator) */

    /* let mut year = 2025;
    year += 1;
    println!("The new year is {year}");

    year -= 5;
    println!("The new year is {year}");

    year *= 2;
    println!("The new year is {year}");

    year /= 2;
    println!("The new year is {year}"); */

    // let is_handsome = true;
    // let is_silly = false;

    // println!("Handsome: {is_handsome}. Silly: {is_silly}");

    // let age: i32 = 40;
    // let is_young = age < 35;
    // println!("{is_young}");
    // println!("{}, {}", age.is_positive(), age.is_negative())

    // let age = 13;
    // let can_see_rated_r_movie = age >= 17;
    // let cannot_see_rated_r_movie = !can_see_rated_r_movie;

    // println!("{}", can_see_rated_r_movie);
    // println!("{}", cannot_see_rated_r_movie);

    // println!("{}", "Coke" == "Pepsi");
    // println!("{}", "Coke" != "Pepsi");
    // println!("{}", "Coke" != "pepsi");
    // println!("{}", 13 == 13.0 as i32);

    // let purchased_ticket = false;
    // let plane_on_time = true;
    // let making_event_and = purchased_ticket && plane_on_time;
    // let making_event_or = purchased_ticket || plane_on_time;
    // println!("{}", making_event_or);
    // println!("{}", making_event_and)

    /*   let first_initial = 'B';
    let emoji: char = '😀';

    println!("{} {}", first_initial.is_lowercase(), emoji.is_uppercase()) */

    // let numbers: [i32; 5] = [42, 3, 22, 10, 23];
    // let apples = ["Granny Smith", "McIntosh", "Envy"];
    // let empty_array: [f64; 0] = [];
    // println!("Length: {}", apples.len());

    // for num in numbers {
    //     println!("{}", num);
    // }

    // let mut seasons = ["Spring", "Summer", "Fall", "Winter"];

    // seasons[2] = "Spring";
    // println!("{}", seasons[2]);

    // let first = seasons[0];
    // let second = seasons[1];
    // println!("The first season is {first} and the second season is {second}");

    // println!("{}", seasons[3]);

    // println!("{seasons:#?}");

    // dbg!(seasons);

    // Tuple
    /* let employee = ("Molly", 22, "Marketing"); */

    // let name = employee.0;
    // let age = employee.1;
    // let department = employee.2;
    /* let (name, age, department) = employee;
    println!("Name: {name}, age: {age}, Department: {department}");
    println!("{employee:#?}") */

    let month_days: std::ops::Range<i32> = 1..31;
    println!("{month_days:?}");
    let month_days: std::ops::RangeInclusive<i32> = 1..=31;
    println!("{month_days:?}");

    for number in month_days {
        println!("{number}");
    }

    let letters: std::ops::Range<char> = 'b'..'f';

    for letter in letters {
        println!("{letter}");
    }

    let colors = ["red", "green", "yellow"];

    for color in colors {
        println!("{color} is a great color.");
    }
}

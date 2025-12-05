/* fn even_or_odd(number: i32) {
    let result = if number % 2 == 0 { "even" } else { "odd" };
    println!("The number is {result}")
} */

// fn countdown(seconds: i32) {
//     if seconds == 0 {
//         println!("Blastoff!")
//     } else {
//         println!("{seconds} seconds to blastoff..");
//         countdown(seconds - 1);
//     }
// }

fn main() {
    // countdown(5);
    // if true {
    //     println!("This line will be output");
    // }

    // if false {
    //     println!("This line will NOT be output");
    // }

    /* let season = "spring"; */

    /* if season == "summer" {
        println!("School's out!");
    } else if season == "winter" {
        println!("Brr, so cold!")
    } else {
        println!("Lots of rain!")
    } */

    /*  match season {
       "summer" => println!("School's out!"),
       "winter" => println!("Brr, so cold!"),
       _ => println!("Lots of rain!"),
    }*/
    /* even_or_odd(9) */

    // let evaluation = true;

    // let value = match evaluation {
    //     true => 20,
    //     false => 40,
    // };
    // println!("{value}");

    // let number = 9;

    // match number {
    //     2 | 4 | 6 | 8 => println!("{number} is even"),
    //     1 | 3 | 5 => println!("{number} is odd"),
    //     _ => println!("unknown"),
    // }

    // match number {
    //     value if value % 2 == 0 => println!("{value} is an even number."),
    //     x if x % 2 != 0 => println!("{x} is an odd number"),
    //     _ => unreachable!(),
    // }

    // let mut seconds = 21;

    /* loop {
        if seconds <= 0 {
            println!("Blastoff!");
            break;
        }

        if seconds % 2 == 0 {
            println!("{seconds} seconds (even number), skipping 3 seconds..");
            seconds -= 3;
            continue;
        }

        println!("{seconds} seconds to blastoff..");
        seconds -= 1;
    } */

    // while seconds > 0 {
    //     if seconds % 2 == 0 {
    //         println!("{seconds} seconds (even number), skipping 3 seconds..");
    //         seconds -= 3;
    //         continue;
    //     }

    //     println!("{seconds} seconds to blastoff..");
    //     seconds -= 1;
    // }
    // println!("Blastoff!");
    //

    /*
    Define a `factorial` function that calculates the
    factorial of a number. The factorial is the product
    of multiplying a number by every incremental
    number leading up to it, starting from 1.

    Examples:
    The factorial of 5 is 5 * 4 * 3 * 2 * 1 = 120
    factorial(5) should return 120.

    The factorial of 4 is 4 * 3 * 2 * 1 = 24
    factorial(4) should return 24.

    Implement two solutions/functions for the problem.
    The first solution should not use recursion.
    The second solution should use recursion.
    */

    // println!("{}", color_to_number("red"));
    println!("{:?}", factorial(5));
    println!("{}", factorial_rec(5))
}

// Without recursion
fn factorial(number: i32) -> i32 {
    let mut result = 1;
    for num in 2..=number {
        result = result * num;
    }
    return result;
}

// With recursion
fn factorial_rec(number: i32) -> i32 {
    if number == 1 {
        return 1;
    } else {
        number * factorial_rec(number - 1)
    }
}

// fn color_to_number(color: &str) -> i32 {
// if color == "red" {
//     return 1;
// } else if color == "green" {
//     return 2;
// } else if color == "blue" {
//     return 0;
// } else {
//     return 0;
//  } */
//
// match
// match color {
//     "red" => return 1,
//     "green" => return 2,
//     "blue" => return 0,
//     _ => unreachable!(),
// }
// }

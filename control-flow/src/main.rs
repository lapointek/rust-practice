/* fn even_or_odd(number: i32) {
    let result = if number % 2 == 0 { "even" } else { "odd" };
    println!("The number is {result}")
} */

fn main() {
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

    /* let mut seconds = 21 */

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

    /* while seconds > 0 {
        if seconds % 2 == 0 {
            println!("{seconds} seconds (even number), skipping 3 seconds..");
            seconds -= 3;
            continue;
        }

        println!("{seconds} seconds to blastoff..");
        seconds -= 1;
    }
    println!("Blastoff!"); */
}

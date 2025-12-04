// apply directive to whole file
// #![allow(unused_variables)]
/* const TAX_RATE: f64 = 7.25; */
//type Meters = i32;
type Season<'a> = &'a str;
type PointsScored = i32;
const TOUCHDOWN_POINTS: i32 = 6;

fn main() {
    //     let apples = 50;
    //     let oranges = 14 + 6;
    //     let _fruits = apples + oranges;
    //     println!(
    //         "This year, my garden has {0} apples and {1} oranges.",
    //         apples, oranges
    //     );

    //     let mut gym_reps = 10;
    //     println!("I plan to do {gym_reps} reps.");
    //     gym_reps = 15;

    // let grams_of_protein = "100.034";
    // let grams_of_protein = 100.345;
    // let grams_of_protein = 100;
    // println!("{}", grams_of_protein)

    /* let income = 10000;
    println!("My income is {income} and my tax rate is {TAX_RATE}"); */

    //let mile_race_length: Meters = 1600;
    //let two_mile_race_length: Meters = 3200;

    let season: Season = "fall";
    let mut points_scored: PointsScored = 28;
    points_scored = 35;

    let event_time = "06:00";
    let event_time = 6;

    println!(
        "Season {0}, Points Scored {1}, Event Time {2}, Touchdown Points {3} ",
        season, points_scored, event_time, TOUCHDOWN_POINTS
    );

    #[allow(unused_variables)]
    let favorite_beverage = "water";
}

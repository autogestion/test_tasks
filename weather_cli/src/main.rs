use std::env;
use rand::Rng;

fn main() {

    let args: Vec<String> = env::args().collect();
    let mut rng = rand::thread_rng();
    let temp = rng.gen_range(-50..50);
    let wind = rng.gen_range(0..100);
    let fl = match wind {
        0..=33 => temp - 3,
        34..=65 => temp - 7,
        _ => temp - 10        
    };

    println!(
        "{0: <10} | {1: <10} | {2: <10} | {3: <10}",
        "city", "temperature", "wind speed", "feels like"
    );
    println!("{0: <10} | {1: <10} | {2: <10} | {3: <10}", &args[1], temp, wind, fl);


}

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let starting_secret = &args[1];
    
    let mut secret = starting_secret.parse::<u64>().unwrap();
    for n in 1..=2000 {
        secret = ((secret * 64) ^ secret) % 16777216;
        secret = ((secret / 32) ^ secret) % 16777216;
        secret = ((secret * 2048) ^ secret) % 16777216;
    }
    println!("{}", secret);

}

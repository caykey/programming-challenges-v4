fn get_prime_factors(mut number: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut largest_prime = 2;
    while largest_prime <= number {
        if number % largest_prime == 0 {
            number /= largest_prime;
            factors.push(largest_prime);
        } else {
            largest_prime += 1;
        }
    }
    factors
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let number = match args.get(1) {
        Some(string) => match string.parse::<u64>() {
            Ok(num) => num,
            Err(_) => panic!("{} is an invalid u64 number!", string),
        },
        None => {
            panic!("Number not specified!");
        }
    };

    let prime_factors = get_prime_factors(number);

    if !prime_factors.is_empty() {
        println!("Prime factors: {:?}", prime_factors);

        println!(
            "Largest prime factor: {}",
            prime_factors.iter().max().unwrap()
        );
    } else {
        println!("No prime factors!");
    }
}

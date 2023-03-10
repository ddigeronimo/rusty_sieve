use std::env;

mod basic;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args.len() > 3 {
        println!("Usage: sieve [LIMIT] [--full-list]");
        return;
    }

    let mut full_list: bool = false;
    if args.len() == 3 {
        if args[2] == "--full-list" {
            full_list = true;
        } else {
            println!("Usage: sieve [LIMIT] [--full-list]");
            return;
        }
    }

    let limit_str: String = args[1].to_owned();
    let limit: u64 = limit_str.parse().unwrap();

    if limit <= 2 {
        println!("There are no primes below {}", limit);
        return;
    }

    if full_list {
        let primes: Vec<u64> = basic::sieve(limit);
        println!("Primes below {}: {:?}", limit, primes);
        return;
    } else {
        let total_count: usize = basic::sieve_count_only(limit);
        println!(
            "There are {} primes below {} in total. To see the full list, use the \"--full-list\" flag",
            total_count,
            limit
        );
    }
}

use std::env;

/// Returns all prime integers up to a given limit
pub fn sieve(limit: u64) -> Vec<u64> {
    let mut numbers_until_limit: Vec<u64> = (0..limit).into_iter().collect();
    let mut markers: Vec<bool> = vec![true; limit as usize];
    markers[0] = false;
    markers[1] = false;

    let mut p: u64 = 2;
    let mut local_p: u64;
    let mut next_p_found: bool;

    loop {
        next_p_found = false;
        local_p = 2 * p;

        while local_p < limit {
            markers[local_p as usize] = false;
            local_p += p;
        }

        for (num, val) in markers.iter().enumerate() {
            let n: u64 = num as u64;
            if n > p && *val {
                p = n;
                next_p_found = true;
                break;
            }
        }

        if !next_p_found {
            break;
        }
    }

    numbers_until_limit.retain(|&x| markers[x as usize]);
    return numbers_until_limit;
}

/// Returns the amount of prime integers up to a given limit, saving some memory compared to returning the list
pub fn sieve_count_only(limit: u64) -> usize {
    let mut markers: Vec<bool> = vec![true; limit as usize];
    markers[0] = false;
    markers[1] = false;

    let mut p: u64 = 2;
    let mut local_p: u64;
    let mut next_p_found: bool;

    loop {
        next_p_found = false;
        local_p = 2 * p;

        while local_p < limit {
            markers[local_p as usize] = false;
            local_p += p;
        }

        for (num, val) in markers.iter().enumerate() {
            let n: u64 = num as u64;
            if n > p && *val {
                p = n;
                next_p_found = true;
                break;
            }
        }

        if !next_p_found {
            break;
        }
    }

    markers.retain(|&x| x);
    return markers.len();
}

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
        let primes: Vec<u64> = sieve(limit);
        println!("Primes below {}: {:?}", limit, primes);
        return;
    } else {
        let total_count: usize = sieve_count_only(limit);
        println!(
            "There are {} primes below {} in total. To see the full list, use the \"--full-list\" flag",
            total_count,
            limit
        );
        return;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sieve_count_only() {
        let actual_prime_count_below_100000: usize = 9592;
        assert_eq!(sieve_count_only(100000), actual_prime_count_below_100000);
    }

    #[test]
    fn test_sieve() {
        let primes_until_100: Vec<u64> = vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97,
        ];
        assert_eq!(sieve(100), primes_until_100);
    }
}

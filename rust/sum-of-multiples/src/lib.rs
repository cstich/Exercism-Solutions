use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut unique_factors = HashSet::new();
    unique_factors.insert(0);
    for f in factors {
        let mut x = 1;
        if f * x == 0 {
            unique_factors.insert(0);
            continue;
        }
        while f * x < limit {
            unique_factors.insert(f * x);
            x += 1;
        }
    }
    unique_factors.iter().sum::<u32>()
}

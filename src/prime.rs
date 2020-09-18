const PRIME_NUMBERS: [u32; 21] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73,
];

/// Returns `None` if val < 2.
pub fn prime_factorization(val: u32) -> Option<Vec<u32>> {
    if val < 2 {
        return None;
    }

    let mut factors: Vec<u32> = Vec::new();
    let mut val = val;

    loop {
        if let Some(teiler) = PRIME_NUMBERS.iter().find(|&&p| val % p == 0) {
            factors.push(*teiler);
            val = val / *teiler;
            if val == 1 {
                break;
            }
        } else {
            factors.push(val);
            break;
        }
    }

    factors.sort();
    Some(factors)
}

pub fn is_prime(num: u32) -> bool {
    if num <= 73 {
        return PRIME_NUMBERS.contains(&num);
    }
    prime_factorization(num).unwrap().len() == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prime_factorization() {
        assert_eq!(prime_factorization(0), None);
        assert_eq!(prime_factorization(1), None);
        assert_eq!(prime_factorization(2), Some(vec![2]));
        assert_eq!(prime_factorization(3), Some(vec![3]));
        assert_eq!(prime_factorization(192), Some(vec![2, 2, 2, 2, 2, 2, 3]));
        assert_eq!(prime_factorization(7676), Some(vec![2, 2, 19, 101]));
    }

    #[test]
    fn test_is_prime() {
        assert_eq!(is_prime(0), false);
        assert_eq!(is_prime(1), false);
        assert_eq!(is_prime(2), true);
        assert_eq!(is_prime(72), false);
        assert_eq!(is_prime(73), true);
        assert_eq!(is_prime(101), true);
    }
}

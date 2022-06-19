pub fn problem() {
    let num = 600851475143;
    let root = (num as f64).sqrt().ceil() as u64;
    let mut greatest = 0;

    for i in 2..root + 2 {
        if num % i == 0 {
            if is_prime(i) {
                greatest = i;
            }
        }
    }
    println!("{}", greatest);
}

fn is_prime(num: u64) -> bool {
    if num < 2 {
        return false;
    }
    if num == 2 {
        return true;
    }
    for i in 2..(num as f64).sqrt().ceil() as u64 + 1 {
        if num % i == 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
#[test]
fn zero() {
    assert_eq!(is_prime(0), false);
}

#[test]
fn two() {
    assert_eq!(is_prime(2), true);
}

#[test]
fn nine() {
    assert_eq!(is_prime(9), false);
}

#[test]
fn eleven() {
    assert_eq!(is_prime(11), true);
}

#[test]
fn some_number() {
    assert_eq!(is_prime(71), true);
}

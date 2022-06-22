pub fn problem() {
    let mut num: u64 = 2;
    for i in 3..21 {
        num *= i as u64;
    }
    let mut is_div: bool = true;
    let list = prime_list((num as f64).sqrt().ceil().sqrt().ceil() as u32);
    for x in list.clone() {
        while is_div {
            for i in 2..21 {
                if (num / x as u64) % i as u64 != 0 {
                    is_div = false;
                    break;
                }
            }
            if is_div {
                num /= x as u64;
            }
        }
        is_div = true;
    }
    println!("{}", num);
}

fn prime_list(a: u32) -> Vec<u32> {
    let mut list: Vec<u32> = vec![2];
    for i in 2..a + 1 {
        let mut prime: bool = true;
        for j in 2..(i as f32).sqrt().ceil() as u32 + 1 {
            if i % j == 0 {
                prime = false;
            }
        }
        if prime {
            list.push(i);
        }
    }
    list
}

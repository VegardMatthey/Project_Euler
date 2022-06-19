pub fn problem() {
    let p = make_palindrome_list();
    let mut greatest = 0;
    for val in p {
        if num(val) != 0 {
            greatest = num(val);
            break;
        }
    }
    println!("{}", greatest);
}

fn make_palindrome_list() -> [u32; 899] {
    let mut pd: [u32; 899] = [0; 899];
    let mut x = 999;
    let mut j = 0;

    while x > 100 {
        let mut string = x.to_string();
        let mut s: String = String::new();
        for i in 0..string.len() {
            s += &string[(string.len() - 1 - i)..(string.len() - i)];
        }
        string += &s;
        pd[j] = string.parse::<u32>().unwrap();

        x -= 1;
        j += 1;
    }
    pd
}

fn num(num: u32) -> u32 {
    let mut x = 0;
    for i in 100..999 {
        if num % i == 0 && (num / i) < 1000 {
            x = num;
        }
    }
    x
}

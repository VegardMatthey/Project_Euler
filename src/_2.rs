pub fn problem() {
    let mut a = 1;
    let mut b = 1;
    let mut buf;
    let mut sum = 0;
    while b < 4000000 {
        buf = b;
        b = a + b;
        a = buf;
        if b % 2 == 0 {
            sum += b;
        }
    }    
    println!("{}", sum);
}
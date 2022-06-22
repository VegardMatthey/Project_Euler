pub fn problem() {
    println!("{}", prime_list(10001)[10000]);
}
fn prime_list(a: u32) -> Vec<u32> {
    let mut list: Vec<u32> = vec![2];
    let mut amount = 0;
    let mut i = 2;
    while amount < a {
        let mut prime: bool = true;
        for j in 2..(i as f32).sqrt().ceil() as u32 + 1 {
            if i % j == 0 {
                prime = false;
            }
        }
        if prime {
            list.push(i);
            amount += 1;
            if amount == a {
                return list;
            }
        }
        i += 1;
    }
    list
}

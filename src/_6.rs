pub fn problem() {
    let sum_squared: u32 = gauss(100) * gauss(100);
    let mut squared: u32 = 0;
    for i in 1..101 {
        squared += i * i;
    }
    println!("{}", sum_squared - squared);
}

fn gauss(num: u32) -> u32 {
    ((num + 1) as f32 * (num as f32 / 2.0)) as u32
}

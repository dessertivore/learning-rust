pub fn problem_1(max_num: i64) -> i64 {
    let mut sum = 0;
    for i in 0..max_num {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }
    return sum;
}

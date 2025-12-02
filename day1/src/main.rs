use std::fs;


fn main() {
    let contents = fs::read_to_string("input")
        .expect("Should have been able to read the file");

    let mut t: i64 = 50;
    let mut count: i64 = 0;

    for line in contents.lines() {
        let (direction, value_str) = line.split_at(1);
        let value = value_str.parse().expect("not valid number");

        match direction {
            "L" => {
                for _ in 0..value {
                    t = (t - 1).rem_euclid(100);
                    if t == 0 { count += 1; }
                }

            },
            "R" => {
                for _ in 0..value {
                    t = (t + 1) % 100;
                    if t == 0 { count += 1; }
                }
            },
            _ => panic!("Invalid input"),
        }
    }
    println!("Part 2 solution is: {}", count);
}

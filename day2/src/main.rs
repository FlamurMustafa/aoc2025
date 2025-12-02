use std::fs;

fn is_valid2(u: u64) -> bool {
    let u_s = u.to_string();

    let len = u_s.len();
    for part  in 1..=len/2 {
        // the element number should be divisible by the length, so it can add up to it by
        // multiplying
        if len % part != 0 {
            continue;
        }
        // how many times to repeat
        let repeat = len / part;
        let par = &u_s[..part];

        if par.repeat(repeat) == u_s {
            return true;
        }
    }
    false
}

fn is_valid(u: u64) -> bool {
    let u_s = u.to_string();

    let (left, right) = u_s.split_at(u_s.len()/2);
    left == right
}

fn process(u: &str) -> (u64, u64) {
    let v_c:Vec<&str> =  u.split('-').collect();

    let left:u64 = v_c[0].parse::<u64>().expect("Can't parse");
    let right:u64 = v_c[1].parse::<u64>().expect("Can't parse");

    let mut sum:u64 = 0;
    let mut sum2:u64 = 0;
    for i in left .. right {
        if is_valid(i) {
            sum += i;
        }
        if is_valid2(i) {
            sum2 += i;
        }
    }

    return (sum, sum2);
}

fn main() {
    let contents = fs::read_to_string("input")
        .expect("Should have been able to read the file");

    let mut sum = 0;
    let mut sum2 = 0;
    for line in contents.lines() {
        let v_c :Vec<&str> = line.split(',').collect();
        for i in &v_c {
            let (s, s2) = process(i);
            sum += s;
            sum2 += s2;
        }
    }
    println!("First part is {}, second part is {}", sum, sum2);
}


use std::fs;
use std::cmp::Ordering;
use std::collections::HashSet;

fn sort_range(contents: String) -> u64 {
    let mut a: Vec<(u64, u64)> = vec![];
    let mut merged_a: Vec<(u64, u64)> = vec![];

    for (i, line) in contents.lines().enumerate() {
        if i == 186 {
            break;
        }
        let lr: Vec<&str> = line.split('-').collect();
        let left = lr[0].parse().expect("Can't turn to uint");
        let right = lr[1].parse().expect("Can't turn to uint");

        a.push((left, right));
    }

    a.sort_by(|x, y| { 
        let (x1, _y1) = x;
        let (x2, _y2) = y;

        if x1 > x2 {
            return Ordering::Greater;
        } else {
            return Ordering::Less;
        }
    });
//3-5        // 3-5
//10-14      // 10-20
//12-18 ---->//
//16-20      //
    for (i, e) in a.iter().enumerate().rev().skip(1) {

    }

    println!("The vec after sort is: {:?}", a);
    123
}

fn main() {
    let contents = fs::read_to_string("input1")
        .expect("Should have been able to read the file");

    //let a = process_ranges(contents.clone());


    //println!("Total sum is {}", check_range(contents, a.clone()));
    //println!("Part 2 total # of fresh ingredients is {}", sort_range(a));
    sort_range(contents);
}

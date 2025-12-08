use std::fs;


fn process(s: &str) -> u32 {
    let mut ln = 0;
    for (i, c) in s.chars().enumerate() {
        let f = c.to_digit(10).unwrap();

        for j in s.bytes().skip(i+1).take(s.len()) {
            let se :u32 = <u8 as Into<u32>>::into(j) - 48;
            if f*10 + se > ln {
                println!("First is {}, second is {}", f, se);
                ln = f*10 +se;
            }

        }
    }
    println!("Larges number for {} is {}", s, ln);
   ln 
}

pub fn process2(batteries: &[u8]) -> u64 {
    let mut sum = 0;
    let mut start = 0;

    for remaining_digits in (0..12).rev() {
        //the end of the sub array in which we will search
        let end = batteries.len() - remaining_digits;

        //slice it from the start (starts at 0, will be updated once we find max from 0 to len -12)
        let slice = &batteries[start..end];

        println!("The main array is {:?}",  batteries);
        println!("The sub slice is {:?}, with len {}", slice, slice.len());

        let (mut max_n, mut max_i) = (0, 0);
        for (idx, &n) in slice.iter().enumerate() {
            if n > max_n {
                max_i = idx;
                max_n = n;
            }
        }

        //times 10 to make way for the new digit
        sum = sum*10 + u64::from(max_n - b'0');
        //update the start to begin from the max leftmost digit
        start += max_i + 1;
    }
    sum
}

fn main() {
    let contents = fs::read_to_string("input")
        .expect("Should have been able to read the file");

    let mut sum = 0;
    for line in contents.lines() {
        sum += process2(line.as_bytes());
    }
    println!("{}", sum);
}

#[doc(hidden)]
mod tests {
    use super::*;
    #[test]
    fn test_2() {
        let i = "987654321111111";
        assert_eq!(process2(i.as_bytes()), 987654321111);
    }
}

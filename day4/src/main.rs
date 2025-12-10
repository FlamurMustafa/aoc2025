use std::fs;


fn find_neigh(arr: Vec<Vec<u8>>) -> (u32, Vec<(usize, usize)>) {

    let mut total_sum = 0;
    let mut marks: Vec<(usize, usize)> = vec![];
    for i in 1..arr.len() -1  {
        for j in 1..arr[i].len() -1  {
            print!("{}", arr[i][j] as char);
            if arr[i][j] == b'@' {
                let mut sum = 0;

                if arr[i-1][j-1] == b'@' { sum += 1; }
                if arr[i-1][j]   == b'@' { sum += 1; }
                if arr[i-1][j+1] == b'@' { sum += 1; }
                if arr[i][j-1]   == b'@' { sum += 1; }
                if arr[i][j+1]   == b'@' { sum += 1; }
                if arr[i+1][j-1] == b'@' { sum += 1; }
                if arr[i+1][j]   == b'@' { sum += 1; }
                if arr[i+1][j+1] == b'@' { sum += 1; }

                if sum < 4 {
                    marks.push((i,j));
                    total_sum += 1;
                }
            }
        }
        println!();
    }
    println!();
    (total_sum, marks)
}

fn solve(mut arr: Vec<Vec<u8>>) -> u32 {

    let mut total_sum = 0;

    loop {
        let (n, neigh) = find_neigh(arr.clone());
        if neigh.len() == 0 {
            break;
        }
        for i in 0..neigh.len() {
            let (xi, yi) = neigh[i];
            arr[xi][yi] = b'.';
        }
        total_sum += n;
    }

    total_sum
}

fn main() {
    let contents = fs::read_to_string("input")
        .expect("Should have been able to read the file");

    let mut arr = vec![vec![0u8; 138]; 138];

    for i in 0..138 {
        arr[0][i] = b'.';
        arr[i][0] = b'.';
        arr[i][137] = b'.';
        arr[137][i] = b'.';
    }

    let s = contents.as_bytes();
    for i in 1..arr.len()-1 {
        for j in 1..arr.len() - 1 {
            arr[i][j] = s[(i-1)*137 + j -1]; 
        }
    }
    println!("The total is {}",  solve(arr));
}



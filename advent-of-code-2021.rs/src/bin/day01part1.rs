
use std::io;
use std::io::BufRead;
use std::iter::Iterator;

fn count_increases<T: Iterator<Item = u64>>(numbers: T) -> u64 {
    let mut prev_n = None;
    let mut num_increases = 0;

    for n in numbers.into_iter() {
        // first input does not count as an increase
        if let Some(p) = prev_n {
            if n > p {
                num_increases += 1;
            }
        }

        prev_n = Some(n);
    }

    num_increases
}

fn main() {
    let stdin = io::stdin();
    let input_numbers = stdin.lock()
        .lines()
        .map(|line|
             line.unwrap().trim()
             .parse::<u64>().expect("error parsing integer"));

    println!("measurement increases: {}", count_increases(input_numbers));
}

#[test]
fn test_count() {
    assert_eq!(count_increases(
        vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263].into_iter()), 7);
    assert_eq!(count_increases(vec![].into_iter()), 0);
    assert_eq!(count_increases(vec![1].into_iter()), 0);
    assert_eq!(count_increases(vec![1,2].into_iter()), 1);
    assert_eq!(count_increases(vec![1,2,3,4].into_iter()), 3);
    assert_eq!(count_increases(vec![1,1,1,4].into_iter()), 1);
    assert_eq!(count_increases(vec![1,2,1,1,2].into_iter()), 2);
    assert_eq!(count_increases(vec![5,3,2,1].into_iter()), 0);
    assert_eq!(count_increases(vec![2,40,50,49].into_iter()), 2);
}



use std::collections::VecDeque;
use std::io;
use std::io::BufRead;
use std::iter::Iterator;

const WINDOW_SIZE: usize = 3;

fn count_window_increases<T: Iterator<Item = u64>>(numbers: T) -> u64 {
    let mut window = VecDeque::with_capacity(WINDOW_SIZE);
    let mut num_increases = 0;

    for n in numbers.into_iter() {
        if window.len() == WINDOW_SIZE {
            let p = window.pop_front().unwrap();
            if p < n {
                num_increases += 1
            }
        }
        window.push_back(n)
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

    println!("measurement increases: {}", count_window_increases(input_numbers));
}

#[test]
fn test_count() {
    assert_eq!(count_window_increases(
        vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263].into_iter()), 5);
    assert_eq!(count_window_increases(vec![].into_iter()), 0);
    assert_eq!(count_window_increases(vec![1].into_iter()), 0);
    assert_eq!(count_window_increases(vec![1,2].into_iter()), 0);
    assert_eq!(count_window_increases(vec![1,2,3].into_iter()), 0);
    assert_eq!(count_window_increases(vec![1,2,3,4].into_iter()), 1);
    assert_eq!(count_window_increases(vec![1,2,3,4,5].into_iter()), 2);
    assert_eq!(count_window_increases(vec![1,1,5,1,1,5,1,1,5].into_iter()), 0);
    assert_eq!(count_window_increases(vec![1,1,5,1,2,3,4,5].into_iter()), 3);
}


// Code originally forked from https://github.com/diptangsu/Sorting-Algorithms/
include!("constants.rs");
use std::time::Instant;

pub fn selection_sort<T: Ord>(array: &mut [T]) {
    let length = array.len();
    for left in 0..length {
        let mut minimal = left;
        for right in (left + 1)..length {
            if array[right] < array[minimal] {
                minimal = right;
            }
        }
        array.swap(minimal, left);
    }
}

fn main() {
    unsafe {
        for _x in 0..50 {
            let start = Instant::now();
            selection_sort(&mut SET2);
            let elapsed = start.elapsed();
            println!("{:?}", elapsed);
        }
    }
}
// Code originally forked from https://github.com/diptangsu/Sorting-Algorithms/
include!("constants.rs");
use std::time::Instant;

pub fn bubble_sort<T: Ord>(array: &mut [T]) {
    for i in 0..array.len() {
        for j in 0..array.len() - 1 - i {
            if array[j] > array[j + 1] {
                array.swap(j, j + 1);
            }
        }
    }
}

fn main() {
    unsafe {
        for _x in 0..50 {
            let start = Instant::now();
            bubble_sort(&mut SET1);
            let elapsed = start.elapsed();
            println!("{:?}", elapsed);
        }
    }
}
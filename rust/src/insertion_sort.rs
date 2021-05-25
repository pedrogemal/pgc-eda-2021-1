// Code originally forked from https://github.com/diptangsu/Sorting-Algorithms/
include!("constants.rs");
use std::time::Instant;

pub fn insertion_sort<T: Ord>(array: &mut [T]) {
    for i in 1..array.len() {
        let mut j = i;
        while j > 0 && array[j] < array[j - 1] {
            array.swap(j, j - 1);
            j = j - 1;
        }
    }
}

fn main() {
    unsafe {
        for _x in 0..50 {
            let start = Instant::now();
            insertion_sort(&mut SET1);
            let elapsed = start.elapsed();
            println!("{:?}", elapsed);
        }
    }
}
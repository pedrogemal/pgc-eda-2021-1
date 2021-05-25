// Code originally forked from https://github.com/diptangsu/Sorting-Algorithms/
include!("constants.rs");
use std::time::Instant;

fn move_down<T: Ord>(array: &mut [T], mut root: usize) {
    let last = array.len() - 1;
    loop {
        let left = 2 * root + 1;
        if left > last {
            break;
        }
        let right = left + 1;
        let child = if right <= last && array[right] > array[left] {
            right
        } else {
            left
        };
        if array[child] > array[root] {
            array.swap(root, child);
        }
        root = child;
    }
}

pub fn heap_sort<T: Ord>(array: &mut [T]) {
    if array.len() <= 1 {
        return;
    }
    let parent = (array.len() - 2) / 2;
    for i in (0..=parent).rev() {
        move_down(array, i);
    }
    for end in (1..array.len()).rev() {
        array.swap(0, end);
        move_down(&mut array[..end], 0);
    }
}

fn main() {
    unsafe {
        for _x in 0..50 {
            let start = Instant::now();
            heap_sort(&mut SET1);
            let elapsed = start.elapsed();
            println!("{:?}", elapsed);
        }
    }
}
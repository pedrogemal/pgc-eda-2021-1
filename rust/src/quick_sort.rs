// Code originally forked from https://github.com/diptangsu/Sorting-Algorithms/
include!("constants.rs");
use std::time::Instant;

fn get_partition<T: Ord>(array: &mut [T], low: isize, high: isize) -> isize {
    let pivot_index = high as usize;
    let mut save_index = low - 1;
    let mut last_index = high;

    loop {
        save_index += 1;
        while array[save_index as usize] < array[pivot_index] {
            save_index += 1;
        }
        last_index -= 1;
        while last_index >= 0 && array[last_index as usize] > array[pivot_index] {
            last_index -= 1;
        }
        if save_index >= last_index {
            break;
        } else {
            array.swap(save_index as usize, last_index as usize);
        }
    }
    array.swap(save_index as usize, pivot_index as usize);
    save_index
}

pub fn quick_sort<T: Ord>(array: &mut [T]) {
    let length = array.len();
    _quick_sort(array, 0, (length - 1) as isize);
}

fn _quick_sort<T: Ord>(array: &mut [T], low: isize, high: isize) {
    if low < high {
        let p = get_partition(array, low, high);
        _quick_sort(array, low, p - 1);
        _quick_sort(array, p + 1, high);
    }
}

fn main() {
    unsafe {
        for _x in 0..50 {
            let start = Instant::now();
            quick_sort(&mut SET2);
            let elapsed = start.elapsed();
            println!("{:?}", elapsed);
        }
    }
}
// Code originally forked from https://github.com/diptangsu/Sorting-Algorithms/
include!("constants.rs");
use std::time::Instant;

fn merge<T: Copy + PartialOrd>(x1: &[T], x2: &[T], y: &mut [T]) {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    while i < x1.len() && j < x2.len() {
        if x1[i] < x2[j] {
            y[k] = x1[i];
            k += 1;
            i += 1;
        } else {
            y[k] = x2[j];
            k += 1;
            j += 1;
        }
    }
    if i < x1.len() {
        y[k..].copy_from_slice(&x1[i..]);
    }
    if j < x2.len() {
        y[k..].copy_from_slice(&x2[j..]);
    }
}

fn merge_sort<T: Copy + Ord>(x: &mut [T]) {
    let n = x.len();
    let m = n / 2;
    if n <= 1 {
        return;
    }
    merge_sort(&mut x[0..m]);
    merge_sort(&mut x[m..n]);
    let mut y: Vec<T> = x.to_vec();
    merge(&x[0..m], &x[m..n], &mut y[..]);
    x.copy_from_slice(&y);
}

fn main() {
    unsafe {
        for _x in 0..50 {
            let start = Instant::now();
            merge_sort(&mut SET1);
            let elapsed = start.elapsed();
            println!("{:?}", elapsed);
        }
    }
}
# Code originally forked from https://github.com/diptangsu/Sorting-Algorithms/

import time
from constants import *
from typing import List

def merge(arr: List[int], aux: List[int], lo: int, mid: int, hi: int) -> None:
    for x in range(lo, hi + 1):
        aux[x] = arr[x]

    i = lo
    j = mid + 1

    for k in range(lo, hi + 1):
        if i > mid:
            arr[k] = aux[j]
            j += 1
        elif j > hi:
            arr[k] = aux[i]
            i += 1
        elif aux[j] < aux[i]:
            arr[k] = aux[j]
            j += 1
        else:
            arr[k] = aux[i]
            i += 1

def aux_sort(arr: List[int], aux: List[int], lo: int, hi: int) -> None:
    if lo >= hi:
        return

    mid = lo + (hi - lo) // 2
    aux_sort(arr, aux, lo, mid)
    aux_sort(arr, aux, mid + 1, hi)
    merge(arr, aux, lo, mid, hi)

def merge_sort(arr: List[int]) -> None:
    aux = [0 for _ in range(len(arr))]
    aux_sort(arr, aux, 0, len(arr) - 1)

if __name__ == '__main__':
    for i in range(50):
        array = set1()
        start = time.time() * 1000
        merge_sort(array)
        end = time.time() * 1000
        elapsed = end - start
        print(elapsed)
    print("---")
# Code originally forked from https://github.com/diptangsu/Sorting-Algorithms/

import time
from constants import *


def bubble_sort(array):
    n = len(array)
    for i in range(n):
        swapped = False
        for j in range(0, n - i - 1):
            if array[j] > array[j + 1]:
                array[j], array[j + 1] = array[j + 1], array[j]
                swapped = True
        if not swapped:
            break


if __name__ == '__main__':
    for z in range(50):
        set_array = set2()
        start = time.time() * 1000
        bubble_sort(set_array)
        end = time.time() * 1000
        elapsed = end - start
        print(elapsed)

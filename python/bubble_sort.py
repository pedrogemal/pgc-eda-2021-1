# Code originally forked from https://github.com/diptangsu/Sorting-Algorithms/

import time
from constants import *

def bubble_sort(item):
    n = len(item)
    for i in range(n):
        swapped = False
        for j in range(0, n - i - 1):
            if item[j] > item[j + 1]:
                item[j], item[j + 1] = item[j + 1], item[j]
                swapped = True
        if not swapped:
            break

if __name__ == '__main__':
    for i in range(50):
        array = set1()
        start = time.time() * 1000
        bubble_sort(array)
        end = time.time() * 1000
        elapsed = end - start
        print(elapsed)
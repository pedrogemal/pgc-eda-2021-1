# Code originally forked from https://github.com/diptangsu/Sorting-Algorithms/

import time
from constants import *


def insertion_sort(array):
    for i in range(1, len(array)):
        key = array[i]
        j = i - 1
        while j >= 0 and key < array[j]:
            array[j + 1] = array[j]
            j -= 1
        array[j + 1] = key


if __name__ == '__main__':
    for z in range(50):
        set_array = set2()
        start = time.time() * 1000
        insertion_sort(set_array)
        end = time.time() * 1000
        elapsed = end - start
        print(elapsed)

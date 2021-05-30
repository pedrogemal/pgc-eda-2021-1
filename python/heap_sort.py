# Code originally forked from https://github.com/diptangsu/Sorting-Algorithms/

import time
from constants import *

def move_down(array, n, root):
    largest = root
    left = 2 * root + 1
    right = 2 * root + 2

    if left < n and array[root] < array[left]:
        largest = left

    if right < n and array[largest] < array[right]:
        largest = right

    if largest != root:
        array[root], array[largest] = array[largest], array[root]
        move_down(array, n, largest)

def heap_sort(array):
    n = len(array)

    for i in range(n, -1, -1):
        move_down(array, n, i)

    for i in range(n - 1, 0, -1):
        array[i], array[0] = array[0], array[i]
        move_down(array, i, 0)

if __name__ == '__main__':
    for i in range(50):
        array = set1()
        start = time.time() * 1000
        heap_sort(array)
        end = time.time() * 1000
        elapsed = end - start
        print(elapsed)

import time
from constants import *
import sys

sys.setrecursionlimit(30000)


def partition(arr, low, high):
    i = (low - 1)
    pivot = arr[high]

    for j in range(low, high):
        if arr[j] < pivot:
            i = i + 1
            arr[i], arr[j] = arr[j], arr[i]

    arr[i + 1], arr[high] = arr[high], arr[i + 1]
    return i + 1


def quick_sort(arr, low, high):
    if low < high:
        pivot_index = partition(arr, low, high)
        quick_sort(arr, low, pivot_index - 1)
        quick_sort(arr, pivot_index + 1, high)


if __name__ == '__main__':
    for z in range(50):
        set_array = set2()
        n = len(set_array)
        start = time.time() * 1000
        quick_sort(set_array, 0, n - 1)
        end = time.time() * 1000
        elapsed = end - start
        print(elapsed)

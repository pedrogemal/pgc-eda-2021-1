# Code originally forked from https://github.com/diptangsu/Sorting-Algorithms/

import time
from constants import *

from typing import List, TypeVar

T = TypeVar("T")


def selection_sort(array: List[T]) -> None:
    for i in range(len(array)):
        minimal = i

        for j in range(i + 1, len(array)):
            if array[minimal] > array[j]:
                minimal = j

        array[i], array[minimal] = array[minimal], array[i]


if __name__ == '__main__':
    for z in range(50):
        set_array = set2()
        start = time.time() * 1000
        selection_sort(set_array)
        end = time.time() * 1000
        elapsed = end - start
        print(elapsed)

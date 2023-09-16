use std::fmt::Debug;

pub trait Sort {
    // default sort method
    fn sort(&mut self);
}

impl<T> Sort for [T]
where
    T: PartialOrd + Copy + Debug,
{
    fn sort(&mut self) {
        self.quick_sort();
    }
}

pub trait BubbleSort {
    // Sorts the vector in-place using bubble sort.
    fn bubble_sort(&mut self);
}
pub trait QuickSort {
    fn _quick_sort(&mut self, left: usize, right: usize);
    fn _quick_sort_len(&self) -> usize;

    fn quick_sort(&mut self) {
        self._quick_sort(0, self._quick_sort_len() - 1);
    }
}
pub trait HeapSort {
    fn heap_sort(&mut self);
}

pub trait MergeSort {
    fn merge_sort(&mut self);
}

impl<T> BubbleSort for [T]
where
    T: PartialOrd + Copy,
{
    /// order: O(n^2)
    fn bubble_sort(&mut self) {
        let mut swapped_flag = true;
        while swapped_flag {
            swapped_flag = false;
            for i in 0..self.len() - 1 {
                if self[i] > self[i + 1] {
                    self.swap(i, i + 1);
                    swapped_flag = true;
                }
            }
        }
    }
}

impl<T> QuickSort for [T]
where
    T: PartialOrd + Copy + Debug,
{
    /// maxorder: O(n^2)
    /// average: O(nlogn)
    fn _quick_sort(&mut self, left: usize, right: usize) {
        if right <= left {
            return;
        }

        let len = right - left + 1;
        let pivot = self[left + len / 2];
        let mut pivot_index = left + len / 2;

        let mut left_now = left;
        let mut right_now = right;

        // left : n <= pivot
        // right: n > pivot
        loop {
            while left_now < right && self[left_now] <= pivot {
                left_now += 1;
            }
            if left_now == right && self[left_now] <= pivot {
                // pivot is max
                self.swap(left_now, pivot_index);
                pivot_index = left_now;
                break;
            }

            while self[right_now] > pivot {
                right_now -= 1;
            }

            // self[right_now] <= pivot
            // self[left_now] > pivot
            if left_now >= right_now {
                // right_now must locate at more right than pivot_index
                // but, the value of right_now is less than pivot
                // so, they can be swapped
                assert!(pivot_index <= right_now);
                assert!(pivot >= self[right_now]);
                self.swap(right_now, pivot_index);
                pivot_index = right_now;
                break;
            }

            assert_ne!(pivot_index, left_now);
            if pivot_index == right_now {
                pivot_index = left_now;
            }

            self.swap(left_now, right_now);
        }
        assert_eq!(self[pivot_index], pivot);

        // values <= pivot | pivot | values > pivot
        if left + 1 < pivot_index {
            self._quick_sort(left, pivot_index - 1);
        }

        if right > pivot_index + 1 {
            self._quick_sort(pivot_index + 1, right);
        }
    }

    fn _quick_sort_len(&self) -> usize {
        self.len()
    }
}

impl<T> HeapSort for Vec<T>
where
    T: Ord + Clone,
{
    /// order: O(nlogn)
    fn heap_sort(&mut self) {
        let mut heap = std::collections::BinaryHeap::from(self.clone());
        for i in (0..self.len()).rev() {
            self[i] = heap.pop().unwrap();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut v = vec![3, 2, 1, 4, 5];
        v.bubble_sort();
        assert_eq!(v, vec![1, 2, 3, 4, 5]);

        let mut v = vec![3, 2, 1, 4, 5, 6, 7, 8, 9, 4, 5, 6, 7];
        v.bubble_sort();
        assert_eq!(v, vec![1, 2, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 9]);
    }

    #[test]
    fn test_quick_sort() {
        let mut v = vec![3, 2, 1, 4, 5];
        v.quick_sort();
        assert_eq!(v, vec![1, 2, 3, 4, 5]);

        let mut v = vec![3, 2, 1, 4, 5, 6, 7, 8, 9, 4, 5, 6, 7];
        v.quick_sort();
        assert_eq!(v, vec![1, 2, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 9]);
    }
    #[test]
    fn test_quick_sort_2() {
        let mut v = vec![8, 2, 9, 8, 1];
        v.quick_sort();
        assert_eq!(v, vec![1, 2, 8, 8, 9]);
    }
    #[test]
    fn test_heap_sort() {
        let mut v = vec![3, 2, 1, 4, 5];
        v.heap_sort();
        assert_eq!(v, vec![1, 2, 3, 4, 5]);

        let mut v = vec![3, 2, 1, 4, 5, 6, 7, 8, 9, 4, 5, 6, 7];
        v.heap_sort();
        assert_eq!(v, vec![1, 2, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 9]);
    }
}

pub trait BinarySearch<T>
where
    T: Ord,
{
    fn partition_point<F>(&self, func: F) -> usize
    where
        F: Fn(&T) -> bool;

    fn lower_bound(&self, target: &T) -> usize {
        self.partition_point(|x| x < target)
    }
    fn upper_bound(&self, target: &T) -> usize {
        self.partition_point(|x| x <= target)
    }

    fn binary_search2(&self, target: &T) -> Option<usize> {
        let lower = self.lower_bound(target);
        let upper = self.upper_bound(target);
        if lower == upper {
            None
        } else {
            Some(lower)
        }
    }
    fn count(&self, target: &T) -> usize {
        let lower = self.lower_bound(target);
        let upper = self.upper_bound(target);
        upper - lower
    }
}

impl<T> BinarySearch<T> for [T]
where
    T: Ord,
{
    // find the first index that func is false
    fn partition_point<F>(&self, func: F) -> usize
    where
        F: Fn(&T) -> bool,
    {
        let mut left = 0;
        let mut right = self.len();
        let mut mid = (left + right) / 2;

        while left < right {
            // left は初めて func が false になる位置
            if func(&self[mid]) {
                left = mid + 1;
            } else {
                right = mid;
            }
            mid = (left + right) / 2;
        }
        left
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_partition_point() {
        let v = vec![2, 4, 3, 3, 3, 5];
        assert_eq!(v.partition_point(|x| x % 2 == 0), 2);
        assert_eq!(v.partition_point(|x| x < &6), 6);
        assert_eq!(v.partition_point(|x| x < &0), 0);
        assert_eq!(v.partition_point(|x| x < &5), 5);
    }

    #[test]
    fn test_lower_bound() {
        let v = vec![1, 2, 3, 3, 3, 4, 5];
        assert_eq!(v.lower_bound(&3), 2);
        assert_eq!(v.lower_bound(&6), 7);
        assert_eq!(v.lower_bound(&0), 0);
    }
    #[test]
    fn test_upper_bound() {
        let v = vec![1, 2, 3, 3, 3, 4, 5];
        assert_eq!(v.upper_bound(&3), 5);
        assert_eq!(v.upper_bound(&6), 7);
        assert_eq!(v.upper_bound(&0), 0);
    }
    #[test]
    fn test_binary_search2() {
        let v = vec![1, 2, 3, 3, 3, 4, 5];
        assert_eq!(v.binary_search2(&3), Some(2));
        assert_eq!(v.binary_search2(&6), None);
        assert_eq!(v.binary_search2(&0), None);
    }
    #[test]
    fn test_count() {
        let v = vec![1, 2, 3, 3, 3, 4, 5];
        assert_eq!(v.count(&3), 3);
        assert_eq!(v.count(&6), 0);
        assert_eq!(v.count(&0), 0);
    }
}

//! Sorting algorithms.

#[cfg(test)]
#[macro_use]
mod test_cases;

pub mod quick_sort;
pub use self::quick_sort::{
    quick_sort, quick_sort_3way, quick_sort_hoare, quick_sort_manual_tco, quick_sort_optimized,
};

pub mod slow_sort;
// pub use self::slow_sort::slow_sort;

pub mod insertion_sort;
pub use self::insertion_sort::insertion_sort;

pub mod selection_sort;
pub use self::selection_sort::selection_sort;

pub mod merge_sort;
pub use self::merge_sort::merge_sort;

pub mod shell_sort;
pub use self::shell_sort::shell_sort;

pub mod bubble_sort;
pub use self::bubble_sort::bubble_sort;

pub mod heap_sort;
pub use self::heap_sort::heap_sort;

pub mod counting_sort;
pub use self::counting_sort::counting_sort;

pub mod bucket_sort;
pub use self::bucket_sort::bucket_sort;

pub mod radix_sort;
pub use self::radix_sort::radix_sort;

pub mod comb_sort;
pub use self::comb_sort::comb_sort;

pub mod gnome_sort;
pub use self::gnome_sort::gnome_sort;

pub mod odd_even_sort;
pub use self::odd_even_sort::odd_even_sort;

pub mod stooge_sort;
pub use self::stooge_sort::stooge_sort;

pub mod cocktail_sort;
pub use self::cocktail_sort::cocktail_sort;


pub fn is_sorted<T>(arr: &[T]) -> bool
    where
        T: std::cmp::PartialOrd,
{
    if arr.is_empty() {
        return true;
    }

    let mut prev = &arr[0];

    for item in arr.iter().skip(1) {
        if prev > item {
            return false;
        }
        prev = item;
    }

    true
}

#[cfg(test)]
mod tests {
    #[test]
    fn is_sorted() {
        use super::*;

        assert!(is_sorted(&[] as &[isize]));
        assert!(is_sorted(&["a"]));
        assert!(is_sorted(&[1, 2, 3]));
        assert!(is_sorted(&[0, 1, 1]));

        assert_eq!(is_sorted(&[1, 0]), false);
        assert_eq!(is_sorted(&[2, 3, 1, -1, 5]), false);
    }
}

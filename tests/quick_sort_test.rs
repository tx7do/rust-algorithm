use rust_algorithm::sorting::quick_sort;
use rust_algorithm::sorting::is_sorted;
mod common;

#[test]
fn test_quick_sort() {
    let mut numbers = [4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
    quick_sort::quick_sort(&mut numbers);
    assert_eq!(numbers, [-31, 0, 1, 2, 2, 4, 65, 83, 99, 782]);
    assert!(is_sorted(&numbers));

    let mut strings = ["beach", "hotel", "airplane", "car", "house", "art"];
    quick_sort::quick_sort(&mut strings);
    assert_eq!(strings, ["airplane", "art", "beach", "car", "hotel", "house"]);
    // assert_eq!(strings, [ "art", "beach", "car", "hotel", "house","car"]);
    assert!(is_sorted(&strings));
}

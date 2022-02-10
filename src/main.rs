pub mod sorting;

fn test_quick_sort() {
    println!("Sort numbers ascending");
    let mut numbers = [4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
    println!("Before: {:?}", numbers);
    sorting::quick_sort::quick_sort(&mut numbers);
    println!("After:  {:?}\n", numbers);

    println!("Sort strings alphabetically");
    let mut strings = ["beach", "hotel", "airplane", "car", "house", "art"];
    println!("Before: {:?}", strings);
    sorting::quick_sort::quick_sort(&mut strings);
    println!("After:  {:?}\n", strings);
}


fn main() {
    test_quick_sort()
}

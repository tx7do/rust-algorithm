/// Slow sort
pub fn slow_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    if len == 0 {
        return;
    }

    _slow_sort(arr, 0, len - 1);
}

fn _slow_sort<T: Ord>(arr: &mut [T], start: usize, end: usize) {
    if start >= end {
        return;
    }

    let m = (start + end) / 2;

    _slow_sort(arr, start, m);
    _slow_sort(arr, m + 1, end);

    if arr[end] < arr[m] {
        arr.swap(end, m);
    }

    _slow_sort(arr, start, end - 1);
}

#[cfg(test)]
mod base {
    use super::*;
    base_cases!(slow_sort);
}

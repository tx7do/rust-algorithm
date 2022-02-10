pub fn cocktail_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();

    if len == 0 {
        return;
    }

    loop {
        let mut swapped = false;

        for i in 0..(len - 1).clamp(0, len) {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }

        swapped = false;

        for i in (0..(len - 1).clamp(0, len)).rev() {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }
    }
}
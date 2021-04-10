pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    let mut ts = 0;
    for i in 0..arr.len() {
        let mut is_sorted = true;
        for j in 0..arr.len() - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                is_sorted = false;
            }
        }
        if is_sorted {
            break;
        }
    }
}

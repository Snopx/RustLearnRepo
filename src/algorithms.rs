use std::fmt::Debug;

pub fn bubble_sort<T: Ord + Debug>(arr: &mut [T]) {
    for i in 0..arr.len() {
        let mut is_sorted = true;
        for j in 0..arr.len() - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                is_sorted = false;
            }
        }
        if is_sorted {
            //decrease the count of outer cycle..
            break;
        }
        println!("index of outer cycle: {},vec like:{:?}", i + 1, &arr);
    }
}
#[test]
fn test_bub(){
    let mut number_list = vec![5, 4, 3, 2, 1];
    println!("before sorted :{:#?}", number_list);
    bubble_sort(&mut number_list);
    println!("after sorted :{:#?}", number_list);
    assert_eq!(vec![1,2,3,4,5],number_list);
}

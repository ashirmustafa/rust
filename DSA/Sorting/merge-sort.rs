fn merge_sort(arr: &mut [i32]) {
    let len = arr.len();
    
    // Base case: if the array has 1 or 0 elements, it's already sorted
    if len <= 1 {
        return;
    }

    // Find the midpoint of the array
    let mid = len / 2;

    // Split the array into two halves
    merge_sort(&mut arr[..mid]); // Sort the left half
    merge_sort(&mut arr[mid..]); // Sort the right half

    // Merge the sorted halves
    let mut temp = arr.to_vec(); // Temporary array to hold the merged result
    merge(&arr[..mid], &arr[mid..], &mut temp[..]);

    // Copy the merged result back into the original array
    arr.copy_from_slice(&temp);
}

// Helper function to merge two sorted halves
fn merge(left: &[i32], right: &[i32], out: &mut [i32]) {
    let mut left_idx = 0;
    let mut right_idx = 0;
    let mut out_idx = 0;

    // Merge the two sorted halves
    while left_idx < left.len() && right_idx < right.len() {
        if left[left_idx] <= right[right_idx] {
            out[out_idx] = left[left_idx];
            left_idx += 1;
        } else {
            out[out_idx] = right[right_idx];
            right_idx += 1;
        }
        out_idx += 1;
    }

    // Copy any remaining elements from the left half
    if left_idx < left.len() {
        out[out_idx..].copy_from_slice(&left[left_idx..]);
    }

    // Copy any remaining elements from the right half
    if right_idx < right.len() {
        out[out_idx..].copy_from_slice(&right[right_idx..]);
    }
}

fn main() {
    let mut arr = [38, 27, 43, 3, 9, 82, 10];
    println!("Original array: {:?}", arr);
    merge_sort(&mut arr);
    println!("Sorted array: {:?}", arr);
}

// Bài tập 1: Cho 2 mảng, kiểm tra mảng này có phải là mảng con của mảng kia không ? (yêu cầu đúng thứ tự của các phần tử)

fn is_equal_arr(arr1:&[i32], arr2:&[i32]) -> bool {
    if arr1.len()!=arr2.len() {
        return false;
    }
    for i in 0..arr1.len() {
        if arr1[i] != arr2[i] {
            return false;
        }
    }
    return true;
}

fn check_sub_arr(org_arr: &[i32], sub_arr: &[i32]) -> bool {
    if org_arr.len() < sub_arr.len() {
        return false;
    }

    for i in 0..(org_arr.len() - sub_arr.len()) {
        if is_equal_arr(&org_arr[i..(i+sub_arr.len())], sub_arr) {
            return true;
        }
    }
    return false;
}

fn main() {
    let org_arr = [1, 2,3,5,6,8, 10, 11];
    let sub_arr = [6,8,10];
    let check_subArr = check_sub_arr(&org_arr, &sub_arr);
    if check_subArr == true {
        println!("Is subarray");
    } else {
        println!("Not subarray");
    }
}

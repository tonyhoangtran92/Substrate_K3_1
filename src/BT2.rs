// Bài tập 2:  Cho 1 chuỗi str Slice như dưới đây. Nhập 1 từ bất kỳ từ bàn phím, in ra số lượng từ này xuất hiện trong chuỗi đã cho. 
// Nâng cao hơn : Tìm kiếm không phân biêt chữ hoa thường, theo dạng regex.
use std::fs;
use std::io;

fn count_substring(org_str: &String, sub_str: &String, is_match_case: bool) -> usize {
    if !is_match_case {
        return org_str.matches(sub_str.as_str()).count(); 
    } else {
        return org_str.to_lowercase().matches(sub_str.to_lowercase().as_str()).count(); 
    }
}

fn main() {
    let bt2_input = fs::read_to_string("src/BT2_input.txt")
        .expect("Cannot read file");
        
    println!("Enter a str to find: ");
    let mut input_str = String::new();
    io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    println!("input = {}", input_str);
    input_str.pop();
    let count = count_substring(&bt2_input, &input_str, true);
    print!("Found {} substring in text.", count);
}

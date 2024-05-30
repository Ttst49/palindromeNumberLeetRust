fn is_palindrome(number: i32) -> bool {
    number.to_string().chars().rev().eq(number.to_string().chars())
}



fn main() {
    let x = 3663;
    let result = is_palindrome(x);
    println!("{}",result)
}


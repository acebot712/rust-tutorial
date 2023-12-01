fn is_palindrome(s: &str) -> bool {
    let clean_str: String = s.chars().filter(|c| c.is_alphanumeric()).collect();
    let reversed_str: String = clean_str.chars().rev().collect();
    clean_str.eq_ignore_ascii_case(&reversed_str)
}

fn main() {
    let s = "A man, a plan, a canal, Panama";
    if is_palindrome(s) {
        println!("{} is a palindrome.", s);
    } else {
        println!("{} is not a palindrome.", s);
    }
}

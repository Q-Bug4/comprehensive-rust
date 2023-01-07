use regex::{Regex};

pub fn luhn(cc_number: &str) -> bool {
    let number = cc_number.replace(" ", "");
    if !validate_number(&number) {
        return false;
    }
    let mut sum = 0;
    for i in 0..number.len() {
        let index = number.len() - i - 1;
        let digit: i32 = number.chars().nth(index).unwrap() as i32 - '0' as i32;
        sum += if i % 2 == 0 {
            digit
        } else {
            let num = 2 * digit;
            num % 10 + num / 10
        };
    }
    sum % 10 == 0
}

fn validate_number(cc_number: &str) -> bool {
    let regex = Regex::new(r"^\d+\d$").expect("Regex error!");
    regex.is_match(cc_number) && cc_number.len() >= 2
}

#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"));
}

#[test]
fn test_empty_cc_number() {
    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("  "));
    assert!(!luhn("    "));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(!luhn("0"));
}

#[test]
fn test_two_digit_cc_number() {
    assert!(luhn(" 0 0 "));
}

#[test]
fn test_valid_cc_number() {
    assert!(luhn("4263 9826 4026 9299"));
    assert!(luhn("4539 3195 0343 6467"));
    assert!(luhn("7992 7398 713"));
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 0569"));
}

#[allow(dead_code)]
fn main() {}
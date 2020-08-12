use super::super::basics::vars::*;

#[test]
fn is_palindrome_false_test() {
    assert_eq!(false, is_palindrome("abc".to_string()));
}

#[test]
fn is_palindrome_true_test() {
    assert_eq!(true, is_palindrome("aba".to_string()));
}
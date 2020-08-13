use super::super::basics::vars::*;

#[test]
fn is_palindrome_false_test() {
    assert_eq!(false, is_palindrome("abc".to_string()));
}

#[test]
fn is_palindrome_true_test() {
    assert_eq!(true, is_palindrome("aba".to_string()));
}

#[test]
fn add_test() {
    assert_eq!(5, add(2, 3));
}

#[test]
fn get_type_test() {
    assert_eq!(String::from("even"), get_type(6).1);
    assert_eq!(String::from("odd"), get_type(5).1);
}

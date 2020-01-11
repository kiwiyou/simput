#[cfg(test)]
mod tests {
    use simput::input;
    #[test]
    fn input_single_line() {
        let line = input!(Line);
        assert_eq!("The quick brown fox jumps over the lazy dog.", line);
    }

    #[test]
    fn input_single_normal_type() {
        let num = input!(i32);
        assert_eq!(13i32, num);
    }

    #[test]
    fn input_multiple_normal_type() {
        let (num1, num2) = input!(i32, f64);
        assert_eq!(13i32, num1);
        assert_eq!(64f64, num2);
    }

    #[test]
    fn input_multiple_mixed_type() {
        let (num1, line1, line2, num2) = input!(i32, Line, Line, f64);
        assert_eq!(13i32, num1);
        assert_eq!("Lorem ipsum", line1);
        assert_eq!("Dolor sit amet", line2);
        assert_eq!(64f64, num2);
    }
}

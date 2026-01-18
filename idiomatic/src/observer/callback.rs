/* Callback function example */

#[allow(dead_code)]
fn callback_fn<F>(callback: F) -> i32
where
    F: Fn() -> i32,
{
    callback()
}

mod tests {
    #[cfg(test)]
    use super::*;

    #[test]
    fn callback_test() {
        let my_callback = || {
            println!("Callback executed!");
            42
        };

        let result = callback_fn(my_callback);
        assert_eq!(result, 42);
    }
}

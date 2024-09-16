fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dummy_test() {
        main();
        assert_eq!(true,true);
    }
}
#[path = "../src/main.rs"] mod main;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dummy_test() {
        main::main();
        assert_eq!(true,true);
    }
}
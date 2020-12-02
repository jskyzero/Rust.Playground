#[cfg(test)]
pub mod tests {
    #[test]
    pub fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    fn it_works2() {
        crate::test::it_works();
    }
}

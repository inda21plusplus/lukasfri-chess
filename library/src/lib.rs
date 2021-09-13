pub mod game;
pub mod pieces;
pub mod structs;

#[cfg(test)]
mod tests {
    use crate::test;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn it_works2() {
        assert_eq!(2, test().unwrap());
    }
}

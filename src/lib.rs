#![allow(dead_code)]

pub mod board;
pub mod movegen;
pub mod piece;

mod types {
    pub use super::board::square::Square;
}

pub mod prelude {
    pub use crate::board::square::Square;
}

pub fn add(left: usize, right: usize) -> usize { left + right }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

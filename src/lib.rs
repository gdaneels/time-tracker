//! # tt crate
//!
//! `tt` stands for time tracker library crate.

mod tt;
pub use tt::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

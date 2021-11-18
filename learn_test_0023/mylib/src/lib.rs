pub mod animal;

#[cfg(test)]
mod tests {
use crate::animal::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn user_cat() {
        // cat::hello();
        assert_eq!(true, cat::is_cat());
    }
    #[test]
    fn user_dog() {
        assert_eq!(true, dog::is_dog());
    }
}

mod character;
mod dice;
mod fight;
mod item;
mod stuff;

pub mod prelude {
    pub use crate::{character::*, dice::*, fight::*, item::*, stuff::*};
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

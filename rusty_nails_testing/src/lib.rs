use mockall_double::double;
use std::ops::Add;

pub mod thinging {
    pub struct Thing {}
    impl Thing {
        pub fn foo(&self) -> i32 {
            42
        }
        pub fn bar() -> i32 {
            32
        }
    }

    #[cfg(test)]
    pub struct MockThing {}
    #[cfg(test)]
    impl MockThing {
        pub fn foo(&self) -> i32 {
            4
        }
        pub fn bar() -> i32 {
            5
        }
    }
}

#[double]
use thinging::Thing;

pub fn do_stuff() -> i32 {
    Thing::bar()
}

#[cfg(test)]
mod rusty_tests {
    use super::*;
    use mockall::*;

    #[test]
    fn my_test() {
        /* This approach of mocking structs causing a namespace problem immediately because Rust doesn't have inheritance and the MockThing cannot be seen as a Thing by do_stuff(). To solve that, we use #[double]. See https://docs.rs/mockall_double/latest/mockall_double/attr.double.html for the details in all their madness.
        mock! {
            pub Thing {
                fn foo(&self) -> i32;
            }
        }
        */
        let mock = Thing {};
        /* Can't do this with the #[double] setup for some reason?! How do the docs not cover fundamental usage?
        mock.expect_foo().returning(|| 3);
        */
        assert_eq!(5, do_stuff());
    }
}

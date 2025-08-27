// rustc claims it doesn't know what lib_labyrinth is from this perspective.
// use lib_labyrinth::modfolder_a;
// also not found; I guess the package name is held apart for special entry point and dependent cases like main.rs and doesn't count as a module from the root crate level lookup?
// use crate::lib_labyrinth::modfolder_a;
// the file lib.rs doesn't get indexed as a module itself, I guess?
// use crate::lib::modfolder_a; // can't find modfolder_a from perspective of lib module inside lib.rs
pub mod my_second_submodule {
    pub fn bar() -> i64 {
        // falling back to the default root level crate 'module' we can reference our nested sibling submodule, but that somewhat removes all the painstaking module setup we did throughout the labyrinth.
        let sum = crate::modfolder_a::submodule_1::my_first_submodule::add(1, 3);
        // seems you basically can't use this approach.
        // let sum = modfolder_a::submodule_1::my_first_submodule::add(1, 3);
        dbg!("sum says {}", sum);
        sum
    }
}

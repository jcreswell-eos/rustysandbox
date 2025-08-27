pub mod modfolder_a;
pub mod top_level_sibling;
pub mod top_level_sibling2;
// pub mod lib; // gives a circular module error
pub mod modfolder_b;

// making an explicit module inside lib.rs to contain the API doesn't really help in providing a common API surface for use inside the lib guts. Basically from that perspective you're stuck with crate on down as if you hadn't done this work already EXCEPT that these mod declarations will get the module indexed relative to top level crate.
// pub mod lib {
// pub mod modfolder_a; // rustc complains it doesn't find modfolder_a module from this perspective.
pub fn foo() -> i64 {
    let sum = crate::modfolder_a::submodule_1::my_first_submodule::add(1, 3);

    dbg!("sum says {}", sum);
    sum
}
// }

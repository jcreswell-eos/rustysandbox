use crate::modfolder_b::submodule_2::my_second_submodule;
use crate::top_level_sibling::whatever;

pub fn thing() {
    whatever();
    let _ = my_second_submodule::bar();
}

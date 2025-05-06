fn main() {
    test_mutate_after_alias();
    test_aliasing_mutable_func_input();
    test_owned_input();
}

/**
 * Looks at how we can and can't interleave aliasing and mutation.
 */
fn test_mutate_after_alias() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &mut i32 = &mut v[2];
    println!("Third element is {}", *num);
    // even though *num gives us an i32, it is considered a place in the context of the borrow checker because the deref operation is looking up the actual data place to get the data value; this is key because it means even seemingly value oriented syntax like the below is subject to the borrow checker because it first goes through a place.
    *num += 1;
    println!("Now the third element is {}", *num);
    v.push(5);
    //println!("but we really shouldn't be able to say yet again that the third element is {} because now the data is mutated.", *num);
    println!("Vector is now {:#?}", v);
}

/**
 * Pushes the boundaries of taking aliases of data to which a mutable reference will eventually become a function input.
 */
fn test_aliasing_mutable_func_input() {
    let mut name = vec![String::from("Ferris"), String::from("Fuzzwick")];
    // you can add all the muts you want; reaching into the vector for some of his data is still an immutable borrow, which is an alias and therefore forbids mutability later. EDIT: the compiler output comes up with a different error than the rust-analyzer red squiggle, saying you cannot borrow name as mutable more than once at a time.
    let first = &mut name[0];
    first.push_str("fork");

    // we can borrow immutably again at this point because we're done with first
    let printedfirst = &name[0];
    println!("first name elem says: {printedfirst}");

    stringify_name_with_title(&mut name);

    // borrow checker forbids using first any-which-way after we may have mutated its container vector and therefore may have invalidated the memory its place points to.
    //println!("{}", first);
}

fn stringify_name_with_title(name: &mut Vec<String>) -> String {
    name.push(String::from("Esq."));
    let full = name.join(" ");
    println!("full name: {full}");
    full
}

/**
 * What happens if we own a foxy Boxy value like Vec, we move it to a function input, and then move it back to the original variable in the function return? Who owns what and what allocations are made?
 */
fn test_owned_input() {
    // todo: this scenario causes a reallocation of the string data, presumably because we're not in the same stack frame so we need to deal with the stack frame owned heap stuff deallocation rule? Or something something flow permission? I didn't really understand that in the place permission docs.
    let mut name = String::from("Ferris");
    println!(
        "before the fun, the string name address says: {:#?}",
        name.as_mut_ptr()
    );
    name = owning_input_returned(name);
    println!(
        "after the fun, the string name address says: {:#?}",
        name.as_mut_ptr()
    );
    println!("the name says: {:#?}", name);

    // this scenario does NOT cause a reallocation of the string data
    let s1 = String::from("stuff");
    println!("before move, s1 address says: {:#?}", s1.as_ptr());
    let s2 = s1;
    /* gives 'borrow of moved value s1' compile error
    println!(
        "after move, s1 address says: {:#?} and s2 address says {:#?}",
        s1.as_ptr(),
        s2.as_ptr()
    );
    */
    println!("after move, s2 address says: {:#?}", s2.as_ptr());
}

fn owning_input_returned(mut name: String) -> String {
    name.push_str("Esq.");
    name
}

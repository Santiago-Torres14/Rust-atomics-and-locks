use std::cell::{Cell, RefCell};

// Cell holds a <T> but its value cannot be borrow
// RefCell not only holds <T> but also a count reference which makes possible to
// safely borrow it and reference 
fn main() {}

fn f_cell(a: &Cell<i32>, b: &Cell<i32>) {
    let before = a.get();
    b.set(b.get() + 1);
    let after = a.get();

    // Might happen since its reference can be internally mutable
    if before != after {
        println!("x");
    }
}

fn f_refcell(v: &RefCell<Vec<i32>>) {
    v.borrow_mut().push(1); // We can modify the value without moving it in and out
}

fn foo(a: &i32, b: &mut i32) {
    let before = *a;
    *b += 1;
    let after = *a;

    // Wont happen because of compiler optimization
    if before != after {
        println!("Happen");
    }

}

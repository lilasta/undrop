extern crate undrop;

use undrop::Undroppable;

struct Struct;

impl Drop for Struct {
    fn drop(&mut self) {
        println!("dropped!");
    }
}

fn main() {
    let undrop = Undroppable::new(Struct);
    Undroppable::drop(undrop);

    let undrop = Undroppable::new(Struct);
    let value = Undroppable::into_inner(undrop);
    drop(value);
}

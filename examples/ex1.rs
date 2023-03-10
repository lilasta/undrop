extern crate undrop;

use undrop::Undroppable;

fn main() {
    let undrop = Undroppable::new(0usize);
    Undroppable::drop(undrop);
}

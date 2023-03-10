# undrop

A wrapper to prevent the compiler from calling T’s destructor. This wrapper is transparent (zero-cost).
It can be thought of as a variant of `core::mem::ManuallyDrop` with compile-time checking.

If you drop a value wrapped by it, the compilation will fail. To drop the value, you must explicitly call `Undroppable::drop`.
Or to drop without calling the destructor, you can use `Undroppable::forget`.
# Notes


## Mutex and RwLock

An `RxLock` (read-write lock) is a `RefCell` with concurrency features added.
The way it works is by blocking the execution thread when the lock has already been acquired,
and wait until the borrowing conflicts are solved.

Mutex works kind of the same approach, with the only difference it doesn't keep track of the
references borrowed.

## Atomics

Atomics types can be interpreted as a concurrent `Cell`, the only way to access
their value os through safe methods provided by the type, and in order to reassign it
one must take it out, modify it to then put it back.

## UnsafeCell

`UnsafeCell` is a primitive type which is used by the rest of the concurrent pointers
for the interior mutability feature.

As its name implies is an unsafe interface, and it shouldn't be used directly, instead
it's ideal to wrap it with a safe interface which makes sure the compiler rules are guarantee


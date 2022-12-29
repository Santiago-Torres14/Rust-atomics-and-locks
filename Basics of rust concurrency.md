# Notes


## Mutex and RwLock

An `RxLock` (read-write lock) is a `RefCell` with concurrency features added.
The way it works is by blocking the execution thread when the lock has already been acquired,
and wait until the borrowing conflicts are solved. This might be compare with a RefCell with
the only difference being that RwLock is Sync.

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
it's ideal to wrap it with a safe interface which makes sure the compiler rules are guarantee.

## Cell

`Cell` is an interface which implements `UnsafeCell` to allow its generic type to have interior mutability.
It cannot be use in multiple threads because it doesn't have the Sync trait, furthermore the only form to
modify its value is by reassign it, what means we must take the interior value and modify it and then set then
value. The previous described functionality is because `Cell` never gives `exclusive reference` unless the underlaying
data is mutable, which might not be the case for the use of the `Cell` interface.

## RefCell

You can consider `RefCell` as a `Cell` but with an exclusive reference for the generic value. Due to that fact,
one can access and mutate the value freely.


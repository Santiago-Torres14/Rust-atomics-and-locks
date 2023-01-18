# Atomics

Atomics in programming helps a process to be indivisibly finished, so when share
among threads, if one attempts to modify an atomic variable, other threads block their
execution if try to modify the value. 

As well atomics need an ordering argument, which helps how to treat memory when acessing,
loading, or modifying the value inside the atomic. 

Now getting more deep into how Rust handles atomics, the first think to notice is the way
its value is modified, one modify the value by a shared reference, which means that it's
making use of `UnsafeCelll`. Futhermore to access the underlying data, each atomic interface
has different methods to operate over it.

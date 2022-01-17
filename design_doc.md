Iron Camel
==========

I plan to design a programming language called Iron Camel, and implement a compiler for it.

Purpose
---------
As the name shows, IronCamel is a mixture of Rust and OCaml (their fun/functional parts). 

No mutability is allowed. No side effect is allowed (with an exception for IO, which will be explained later).

No loops, only recursions. There will be map, reduce and filter for list operations.


IO
------
There will be a read zone and write zone. In read zone, data will be loaded by a specifid schema. In write zone, only writeStr is allowed.
This is inspired by Pig Script for Hadoop.

Outside read/write zones, no operations with side effects are allowed.

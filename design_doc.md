Iron Camel
==========

I plan to design a programming language called Iron Camel, and implement a compiler for it.

Purpose
---------
IronCamel is the "most functional" imperative programming language
As the name shows, IronCamel is a mixture of Rust and OCaml (their fun/functional parts). 

No mutability is allowed. No side effect is allowed (with an exception for IO, which will be explained later).

No loops, only recursions. There will be map, reduce and filter for list operations.


IO
------
IO has an analogy, Wormhole in ironCamel. Everything in ironCamel (our universe) is functional and immutable. However, external data may bump into/from our program caused by IO. ironCamel has special syntax and semantics for IO operations.

Syntax - Inspired by C++
readline@stdin >> s;
writeln@stdout << result; 
There will be a read zone and write zone. In read zone, data will be loaded by a specific schema. In write zone, only writeStr is allowed.
This is inspired by Pig Script for Hadoop.

Outside read/write zones, no operations with side effects are allowed.



Syntax
----------
```
(* There needs to be at least one function as the start point *)
program = { function }, function;
function = "fn", identifier, "(", argumet_list, ")", block;

block = "{",
		{ statement },
		expression, "}" ;

(* what if I want to add type annotation here *)
argument_list = empty
    | identifier, { ",", identifier };

statement = assignment
	| io_operation
	| expression, ";" 
	| ";" ;
	(* An expression is guaranteed to be side-effect free. In this case, nothing would happen *)
io_operation = STUB; (* Thinking of how to define it*)
assignment = "let", identifier, "=", expression, ";" ;
(* No shadowing is allowed *)

(* definition of expression is most complex *)
expression = literal
	| if_else_expression
	| call_a_collable_object
	| closure
	;
	
if_else_expression = "if", expression, "then", block, "else", block;

call_a_collable_object = identifier, '(', argumet_list, ')';

(* Maybe I could make the block more powerful?  *)



literal = booleanLiteral
	| natural_number ;  (* Leading-zero not allowed for positive integers *)
	(* un finished *)
digit_excluding_zero = "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" ;
digit                = "0" | digit_excluding_zero ;
natural_number = digit_excluding_zero, { digit } ;

(* integer = "0" | [ "-" ], natural number ;
not implemented *)
```


Types
=========

Function's argument
----------------
I'm not designing a syntax for explicit type for argument yet.


Design
=====


Scoping
-----
No C-style global variables. No mutually recursion. How can I force it?

When discussing definition, there IS a global scope - only for functions. All functions are in the top-level, vice versa.





Structure
=======

The shipped toolchain has three rings

1. compiler (implemented in another PL, privileged, as small as possible)
2. core (in IronCamel, some privileges, may access some APIs not implemented in ironCamel)
3. stdlib (in IronCamel, no privilege, no access to non-ironCamel code)

Obviously, we want to keep only necessary things in ring 1.





Identifier
-----

First letter: upper/lowercase letter, underscore

Others: also numbers



Stream
-------

I want to implement Stream as the basic data structure. A stream may be finite or infinite. We can apply head to take the first element, or use tail for the remaining.

`take` can be implemented in `core`



How may I express the constructor of an infinite stream? I think I should read https://www.cs.cornell.edu/courses/cs3110/2018sp/l/12-streams/notes.html

https://www.cs.cornell.edu/courses/cs3110/2011sp/Lectures/lec24-streams/streams.htm



Closures
--------

|param| expr

All elements will be borrowed by immutable reference.

Only single expr is allowed (will this cause trouble?)





Some IO functions (in core or stdlib?)
-------

Read next integer, read n integers

(read a csv?)







Some questions
-----

How should I implement closure? Should I auto catch all variables?

If so, how to do function currying?

Indirect recursion should be banned in ironCamel


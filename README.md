IronCamel
==========

How to run
-------------
```
cargo build
target/debug/ironcamel --include include/core.icml -i include/stdlib.icml  --run example/fac.icml 
```


Purpose
---------
IronCamel is the "most functional" imperative programming language. It's inspired by Rust, OCaml and Lisp.

No mutability is allowed in IronCamel. All **expressions** have no side effect. There is an exception for IO.

No loops in IronCamel, only recursions.

This project was originally the final project for [CS642](https://cs.uwaterloo.ca//current/courses/course_descriptions/cDescr/CS442) at UW. My submitted version is [v0.1](https://github.com/Endle/ironCamel/tree/v0.1).   



IO
------
IO is the Wormhole in ironCamel. Everything in ironCamel (our universe) is functional and immutable. However, external data may bump into/from our program caused by IO. ironCamel has special syntax and semantics for IO operations.
```
Syntax - Inspired by C++
readline@stdin >> s;
writeline@stdout << result;
fopen_read @ fin = "example/magic_number";
```

See examples: `file_io` `file_io_write`, `io_read_then_sort`


Scoping
-----
No C-style global variables. Only functions are allowed in global scopes. Obviously, all functions are guaranteed to be pure functions.


Runtime Structure
----------------
The shipped toolchain has three rings

1. builtin (implemented in Rust, privileged)
2. core (in IronCamel)
3. stdlib (in IronCamel)

We want to keep only necessary things in Ring 1.


Built-in functions
--------------
`cons`: Receive an element, and a list. See https://ocaml.org/api/List.html
`list`: Receive zero or more elements, build a list. This is the privilege of built-in functions to use a variable number of parameters.
`hd`: Get first element.
`tl`: Get a list for remaining elements.
`is_empty`
`atoi`
`strtok`

Syntax
===============

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
	| io_operation, ";"  ;
	(* An expression is guaranteed to be side-effect free. 
	I would disallow a statement with only an expression, or an empty statement *)
	
io_operation = write_operation
    | read_operation
    | open_file_operation;
    
read_operation =  identifier, "@", identifier, ">>", identifier;
write_operation = identifier, "@", identifier, "<<", expression;
open_file_operation = identifier, "@", identifier, "=", string;

assignment = "let", identifier, "=", expression, ";" ;
(* No shadowing is allowed *)

(* definition of expression is most complex *)
expression = literal
	| if_else_expression
	| call_a_collable_object
	| closure
	;
	
if_else_expression = "if", expression, "then", block, "else", block;

call_a_collable_object = callee_name, '(', argumet_list, ')';

closure = "|", argument_list, "|", block;

callee_name = identifier
    | arithmetic_operator;
arithmetic_operator = "+" | "-" | "*" | "==" | ">" | "<" | "<=" | ">=";



literal = booleanLiteral
	| natural_number   (* Leading-zero not allowed for positive integers *)
	| stringLiteral ;
digit_excluding_zero = "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" ;
digit                = "0" | digit_excluding_zero ;
natural_number = digit_excluding_zero, { digit } ;


```


Some thoughts about my design
===============================

Why I want to create such a PL
----------------------------
The main reason is that I love Rust, but I had to admit that Rust is a very complex language. If we remove the lifetime, the ref/deref rule from Rust, leaving only the fun parts of Rust[1], is this still a good language?

My answer is yes. I'm doing this experiment with Ironcamel, which is inspired by Rust, OCaml and Lisp.

[1] Yes, I want to mention the book JavaScript: The Good Parts here.

Worry-free function calls
--------------------------
This is a term I informally created. I'll explain it with pseudo-code
i. I created a complex object objA
ii. let objB = ThirdPartyLib.foo(objA)
iii. Access objA for another purpose

Here is the problem. How can I know whether objA is changed or not? I
need to be cautious when calling third-party libraries when writing Java
or Python code. The lib may come from a badly maintained opensource
project, or the company's internal codebase, which is committed under
pressure of a deadline. The lib may edit objA without explicitly
mentioning it in docs. If I'm writing in C++, it is a bit better. I can
check the function signature. foo(const &a) and foo(&a) are different.

For Rust, I can call ThirdPartyLib.foo worry-free. If objA is moved
away, I know I need to copy it myself if needed. I can always create
an immutable reference of objA. If the lib asks me to make a mutable
reference, then I know I need to be cautious about how the third-party
lib would edit my object.


In IronCamel, everything is immutable (except IO), so **worry-free** is trivial and straightforward here.


Why No Loops
------------------------
The idea about **prohibiting loops** came from my experience of teaching programming. The memory model, the mutability of variables, passing the value, pointer or reference when calling a function[2]... So many concepts are rushing to learners' brains, and they still need to remember to add `i++` at the end of the `while` loop.

In Ironcamel, only recursion is allowed. I wish Ironcamel would be closer to math definitions (like Fibonacci numbers)

[2]: Some universities are teaching C++ for programming ABC. I don't think this is a good idea.











Iron Camel
==========

I plan to design a programming language called Iron Camel, and implement an interpreter for it.

Purpose
---------
IronCamel is the "most functional" imperative programming language.
No mutability is allowed. No side effect is allowed (with an exception for IO, which will be explained later).

No loops, only recursions.

IO
------
IO is the Wormhole in ironCamel. Everything in ironCamel (our universe) is functional and immutable. However, external data may bump into/from our program caused by IO. ironCamel has special syntax and semantics for IO operations.

Syntax - Inspired by C++
readline@stdin >> s;
writeline@stdout << result; 


TODO: How to open a new file

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
	| io_operation, ";"  ;
	(* An expression is guaranteed to be side-effect free. 
	I would disallow a statement with only an expression, or an empty statement *)
	
io_operation = write_operation
    | read_operation;
    
read_operation =  identifier, "@", identifier, ">>", identifier;
write_operation = identifier, "@", identifier, "<<", expression;

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




Design
=====


Scoping
-----
No C-style global variables. Only functions are allowed in global scopes.


Runtime Structure
----------------
The shipped toolchain has three rings

1. builtin (implemented in another PL, privileged)
2. core (in IronCamel)
3. stdlib (in IronCamel)

Obviously, we want to keep only necessary things in ring 1.


Built-in functions
--------------
`cons`: Receive an element, and a list. See https://ocaml.org/api/List.html
`list`: Receive zero or more elements, build a list. This is the privilege of built-in functions to use a variable number of parameters.
`hd`: Get first element.
`tl`: Get a list for remaining elements.
`is_empty`






Closures
--------

|param| expr

All elements will be borrowed by immutable reference.



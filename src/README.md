# Source Files

## ~ast.rs~

Provides two enums that are the result of parsing and the input to the
interpreter. `Statements` can be anything from loops to print and `Expressions`
are code that evaluates to the primitive types. All Statements result in
Expressions and All expressions result in types.

## `environment.rs`

Contains the code for scope and closures that determines variable availability.

## `eval.rs`

The interpreter.

## `lexer.rs`

Splits a source string into a vector of `Token`.

## `main.rs`

REPL and file handling.

## `parser.rs`

Turns a vector of tokens into a vector of statements

## `tokens.rs`

Provides the enum `Token`.

## `typechecker.rs`

Checks a vector of statements for type inconsistencies and returns a vector of
types for each statement.


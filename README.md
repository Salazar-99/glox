# glox
A Rust implementation of the Lox programming language from Crafting Interpreters

## TODO
- Fix prompt
- Wire up intepreter to Glox struct
- Add proper error handling and reporting in both the parser and interpeter

## Notes
This implementation of Glox consists of a Scanner -> Recursive-Descent Parser -> Tree-Walk Interpreter
Lexical grammar - The rules for parsing the language into Tokens
Syntactic grammar - The rules for parsing Tokens into Expressions

Precedence rules are needed to avoid ambiguity when parsing expressions.
Rules with higher precedence are applied first (e.g. evaluate division before subtraction).
Rules with higher precedence are also said to "bind tighter"
The predence rules for Lox, from highest to lowest, are
- Equality
- Comparison
- Term
- Factor
- Unary (the only right associatve operator)



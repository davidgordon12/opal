# Opal Dev Log

Development for Opal began Tuesday, December 19 01:43:42 EST 2023. All previous commits were from a C stack-based, bytecode VM, interpreted language, previously named Mira.

## 2024-01-01
First entry in the Opal Dev Log. Up to this point we had created a lexer that supports each and every kind of token that I aim to support, and the beginning of a parser, that is able to create an AST of binary expressions. Opal currently supports the following binary operations on Identifiers and Numbers;
Addition,
Subtraction,
Multiplication,
Division,
Exponents,
Modulo. Of course opal operations have differnet precedence, and as such is BEDMAS (or PEMDAS) compliant. Our compiler can compile those simple operands (on integers) one at a time. Our next step will be to walk the AST so that we are able to compile more complex expressions. Afterwards I want to support floating point numbers, before going back to our parser to implementfunctions and variables.

## 2024-01-02
Our compiler can now accept multiple files from the user and compile them sequentially, outputting .asm files for each from the user. Opal will not assemble or link the outputs, that is up to the user now. In the future I may create a build system if it's ever necessary.
We also are able to traverse our AST to compile complex binary expressions, by simply recursing through the tree of BinaryExpressions. In the future we will need pattern matching to decipher what we are actually compiling. Currently our compiler generates static code based on what method we call, but we want to use the results of previous operations for the next one, so I will need to figure out how to get that working.

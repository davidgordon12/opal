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

## 2024-01-04
I optimized the basic binary operations a bit by immediately doing the operation with one value in a register, and the second value as a constant, rather than moving both values into registers then operating. However this approach doesn't seem to work because I have no way to move the result of this operation up to the next operation, if it is nested somewhere lower in the tree. Our single pass compiler generates code on the fly and we have no way of knowing (yet) whether or not we are compiling a simple expression or a large tree of expressions.
Take for example;
2+3
This is trivial as we can just move 2 into rax, and directly add rax with 3. But what if the operation looked like this;
2+3*3
First we need to multiply 3*3. We can do the same thing and move 3 into rax, then mul rax, 3. Only one move! But how do we bring the result from the right side of our operation into the left side? Of course you could get the result in our program and then compile it but then we are just writing an interpreter. The simplest way I can think of is to use a stack and push / pop values as we need them. This is a lot slower than what I was intending to do with just registers but I do not see any other way at the moment.

## 2024-01-08
My previous optimizations actually broke the div and mul instructions because you cannot provide a constant to that operation, only a value from a register. I am currently implementing the stack solution I mentioned in my previous entry and so far have addition and subtratction working. I am debating on whether or not I want to link with the C standard library to make my life easier, this will require a few simple tweaks but it is better to decide early on than later.

### Continued;
Complex binary expressions now work, with a brute force approach of pushing each value or result of an operation to the stack, then popping two values each time. In the future we can do a few more passes to optimize this but that will not even be considered until the rest of the check list is done.

## 2024-01-09
The global entry point is now "main", and I'm linking with glibc just so I can get Opal off the ground a bit quicker. We have implemented print for digits using glibc's printf, and I plan on adding support for more glibc functions, but first I plan on doing a small refactor of our code generation (compiler.rs).

## 2024-03-14
The language has pivoted to be a stack-based interpreted language rather than a compiled one, since I have realized I am simply not ready to take on such a task, this way it will be a lot more managable with my current knowledge. In the future, I still plan on making a compiler for the language, since it will always be small. I have begun rewriting the parser from scratch. I previously was following a couple tutorials but I realized mixing the code from a few people into my own wouldn't workout well, so I plan on just rolling my own, tackling the syntax slowly. It won't be ideal or follow any "best practice" patterns, but it will be my own and work for my language.

## 2024-03-16
More work on the parser. Able to now parse functions, their return types, and their parameters. Also added print and return statements since they're the first things I will get running in the VM, and the easiest way to get feedback from any running code. I'm going to finish off function calls then begin work on the runtime environment.

## 2024-12-25
Binary expressions seem to be evaluated out of order. The let.opal example program evaluates (hello + word) to (wordhello)

## 2024-12-26
Fixed Binary Expr issue, which only affected strings due to me appending them in the wrong order. Added more clear error messages to the parser and vm execution. Added the ability to call functions and pass parameters, which also means we can now use recursion (once I implement conditionals so we can have a base case).

## 2024-12-27
Implemented if statements, but now the issue is properly returning or exiting a function when I want it to.

## 2024-12-28
Need to introduce scope for variables as well. I figured out I actually didn't implement the ability to pass variables to methods, it is simply using the pre-declared variables outside of the method body, inside of the method itself, since there is no concept of scope yet.
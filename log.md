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

proc fib(a, b) -> Number {
	let c = a + b;
	fib(b, c);
}

let a = 1;
let b = 2;
fib(a, b);

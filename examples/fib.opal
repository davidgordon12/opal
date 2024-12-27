proc fib(a, b) -> Number {
	let c = a + b;
	if c > 20 {
		return c;
	}
	fib(b, c);
}

let a = 1;
let b = 2;
fib(a, b);

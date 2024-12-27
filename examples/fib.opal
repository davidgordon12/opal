proc fib(a, b) -> Number {
	let c = a + b;
	print c;
	if c > 3 {
		return c;
	}
	fib(b, c);
}

let a = 1;
let b = 2;
fib(a, b);

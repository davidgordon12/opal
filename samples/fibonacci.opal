def fib(n: i32) -> void {
  let a: i32 = 0;
  let b: i32 = 1;

  let i: i32 = 0;
  while i < n {
    let c: i32 = a + b;
    println c;

    a = b;
    b = c;

    i++;
  }
}

def main() -> void {
  fib(15);
}
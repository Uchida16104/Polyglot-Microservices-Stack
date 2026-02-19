fn fibonacci(n: Int) -> Int:
    if n <= 1:
        return n
    var a: Int = 0
    var b: Int = 1
    for _ in range(2, n + 1):
        let c = a + b
        a = b
        b = c
    return b

fn is_prime(n: Int) -> Bool:
    if n < 2:
        return False
    var i: Int = 2
    while i * i <= n:
        if n % i == 0:
            return False
        i += 1
    return True

fn main():
    print("Mojo | fib(15)=", fibonacci(15), end="")
    print(" | primes<50:", end="")
    for i in range(2, 50):
        if is_prime(i):
            print("", i, end="")
    print()

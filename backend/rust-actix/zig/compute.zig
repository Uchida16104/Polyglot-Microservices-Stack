const std = @import("std");

fn fibonacci(n: u64) u64 {
    if (n <= 1) return n;
    var a: u64 = 0;
    var b: u64 = 1;
    var i: u64 = 2;
    while (i <= n) : (i += 1) {
        const c = a + b;
        a = b;
        b = c;
    }
    return b;
}

fn isPrime(n: u64) bool {
    if (n < 2) return false;
    var i: u64 = 2;
    while (i * i <= n) : (i += 1) {
        if (n % i == 0) return false;
    }
    return true;
}

pub fn main() !void {
    const stdout = std.io.getStdOut().writer();
    try stdout.print("Zig | fib(15)={d} | primes<50:", .{fibonacci(15)});
    var i: u64 = 2;
    while (i < 50) : (i += 1) {
        if (isPrime(i)) try stdout.print(" {d}", .{i});
    }
    try stdout.print("\n", .{});
}

using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;

class Program
{
    static long Fibonacci(int n)
    {
        if (n <= 1) return n;
        long a = 0, b = 1;
        for (int i = 2; i <= n; i++) { long c = a + b; a = b; b = c; }
        return b;
    }

    static bool IsPrime(long n)
    {
        if (n < 2) return false;
        for (long i = 2; i * i <= n; i++)
            if (n % i == 0) return false;
        return true;
    }

    static void Main()
    {
        var primes = Enumerable.Range(2, 48)
                               .Where(n => IsPrime(n))
                               .ToList();
        var sb = new StringBuilder();
        sb.Append($"C# | fib(15)={Fibonacci(15)} | primes<50: ");
        sb.Append(string.Join(", ", primes));
        Console.WriteLine(sb.ToString());
    }
}

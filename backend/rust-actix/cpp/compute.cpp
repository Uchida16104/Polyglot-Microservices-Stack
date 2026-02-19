#include <iostream>
#include <vector>
#include <cmath>
#include "compute.h"

extern "C" long long fibonacci(int n) {
    if (n <= 1) return n;
    long long a = 0, b = 1;
    for (int i = 2; i <= n; ++i) {
        long long c = a + b;
        a = b;
        b = c;
    }
    return b;
}

extern "C" int is_prime(long long n) {
    if (n < 2) return 0;
    for (long long i = 2; i <= (long long)std::sqrt((double)n); ++i)
        if (n % i == 0) return 0;
    return 1;
}

int main() {
    std::vector<int> primes;
    for (int i = 2; i < 50; ++i)
        if (is_prime(i)) primes.push_back(i);

    std::cout << "C++ | fib(15)=" << fibonacci(15) << " | primes<50:";
    for (int p : primes) std::cout << " " << p;
    std::cout << std::endl;
    return 0;
}

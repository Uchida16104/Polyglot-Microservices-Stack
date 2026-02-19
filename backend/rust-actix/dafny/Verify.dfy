method Fibonacci(n: nat) returns (result: nat)
  ensures result >= 0
{
  if n == 0 {
    result := 0;
    return;
  }
  if n == 1 {
    result := 1;
    return;
  }
  var a: nat := 0;
  var b: nat := 1;
  var i: nat := 2;
  while i <= n
    invariant 2 <= i <= n + 1
    decreases n + 1 - i
  {
    var c := a + b;
    a := b;
    b := c;
    i := i + 1;
  }
  result := b;
}

method IsPrime(n: nat) returns (result: bool)
{
  if n < 2 {
    result := false;
    return;
  }
  var i: nat := 2;
  result := true;
  while i * i <= n
    decreases n * n - i * i + 1
  {
    if n % i == 0 {
      result := false;
      return;
    }
    i := i + 1;
  }
}

method Main()
{
  var f10 := Fibonacci(10);
  var p17 := IsPrime(17);
  print "Dafny | fib(10)=", f10, " | is_prime(17)=", p17, "\n";
}

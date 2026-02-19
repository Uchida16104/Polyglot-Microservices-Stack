module Verify

open FStar.Mul

val factorial : nat -> Tot nat
let rec factorial n =
  if n = 0 then 1
  else n * factorial (n - 1)

val factorial_positive : n:nat -> Lemma (factorial n >= 1)
let rec factorial_positive n =
  if n = 0 then ()
  else factorial_positive (n - 1)

val fibonacci : nat -> Tot nat
let rec fibonacci n =
  if n <= 1 then n
  else fibonacci (n - 1) + fibonacci (n - 2)

val fibonacci_nonneg : n:nat -> Lemma (fibonacci n >= 0)
let rec fibonacci_nonneg n =
  if n <= 1 then ()
  else begin fibonacci_nonneg (n-1); fibonacci_nonneg (n-2) end

let _ =
  let r1 = factorial 5 in
  let r2 = fibonacci 10 in
  FStar.IO.print_string (
    "F* | factorial(5)=" ^ string_of_int r1 ^
    " | fib(10)="        ^ string_of_int r2 ^ "\n"
  )

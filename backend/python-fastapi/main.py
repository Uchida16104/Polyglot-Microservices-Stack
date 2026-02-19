from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
from pydantic import BaseModel
import math
import time
from typing import Any, Dict, List

app = FastAPI(title="Polyglot Python FastAPI", version="1.0.0")

app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_methods=["*"],
    allow_headers=["*"],
)


class ComputeResponse(BaseModel):
    language: str
    result: str
    duration_ms: float
    extras: Dict[str, Any] = {}


def fibonacci(n: int) -> int:
    if n <= 0:
        return 0
    a, b = 0, 1
    for _ in range(2, n + 1):
        a, b = b, a + b
    return b


def sieve_of_eratosthenes(limit: int) -> List[int]:
    if limit < 2:
        return []
    sieve = bytearray([1]) * (limit + 1)
    sieve[0] = sieve[1] = 0
    for i in range(2, int(math.isqrt(limit)) + 1):
        if sieve[i]:
            sieve[i * i :: i] = bytearray(len(sieve[i * i :: i]))
    return [i for i, v in enumerate(sieve) if v]


@app.get("/compute", response_model=ComputeResponse)
async def compute():
    t = time.perf_counter()
    primes = sieve_of_eratosthenes(50)
    fib15 = fibonacci(15)
    duration = (time.perf_counter() - t) * 1000
    return ComputeResponse(
        language="Python3/FastAPI",
        result=f"Python3 | fib(15)={fib15} | primes<=50: {primes}",
        duration_ms=round(duration, 4),
        extras={"primes_count": len(primes), "fib15": fib15},
    )


@app.get("/health")
async def health():
    return {"status": "healthy", "lang": "python3", "framework": "fastapi"}

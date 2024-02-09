from fractions import Fraction
from typing import List
import test


def fractran2(code: str, n: int) -> int:
    c = code.split(" ")
    x: List[Fraction] = [Fraction(x) for x in c]
    n = Fraction(n)

    while True:
        for f in x:
            i = f * n
            # if i is an integer, break
            if i.denominator == 1:
                print(f"{i} is integer with {f}")
                n = i
                found_nothing = False
                break
            found_nothing = True
        if found_nothing:
            break
    print(f"found {n}")
    return int(n)

def fractran(code: str, n: int) -> int:
    c = code.split(" ")
    x: List[Fraction] = [Fraction(x) for x in c]
    n = Fraction(n)
    cycles = 0
    while True:
        cycles += 1
        for f in x:
            i = f * n
            # if i is an integer, break
            if i.denominator == 1:
                    # print(f"{i} is integer with {f}")
                n = i
                found_nothing = False
                break
            found_nothing = True
        if found_nothing or cycles > 1000:
            break
    return int(n)


TESTS = [
    # Adder:  2^a * 3^b  ->  3^(a+b)
    (3**9, 2**4 * 3**5, "3/2"),
    # Multiplier:  2^a * 3^b  ->  5^(a*b)
    (5**12, 2**3 * 3**4, "455/33 11/13 1/11 3/7 11/2 1/3"),
    # Floordiv/Remainder:  2^n * 3^d * 11  ->  5^q * 7^r  where  n = q*d + r  and  0 <= r < d
    (5**2 * 7**1, 2**7 * 3**3 * 11, "91/66 11/13 1/33 85/11 57/119 17/19 11/17 1/3"),
    # Hamming weight:  2^a  ->  13^H(a)  where H(a) = number of ones in binary expansion of a
    (13**3, 2**11, "33/20 5/11 13/10 1/5 2/3 10/7 7/2"),
    # Fibonacci:  78 * 5^(n-1)  ->  2^Fn  where Fn = nth fibonacci number
    (
        2**13,
        78 * 5 ** (7 - 1),
        "17/65 133/34 17/19 23/17 2233/69 23/29 31/23 74/341 31/37 41/31 129/287 41/43 13/41 1/13 1/3",
    ),
    # Prime Generator:  10  ->  stream mixed with 10^n  where  n is prime
    (127381246468476, 10, "7/3 99/98 13/49 39/35 36/91 10/143 49/13 7/11 1/2 91/1"),
]

for expected, n, code in TESTS:
    print(f"fractranRes: {fractran(code, n) == expected}")


import random
import re
import string

import myrustlib  # <-- Import the Rust implemented module (myrustlib.so)

# def count_doubles(val):
#     """Count repeated pair of chars ins a string"""
#     total = 0
#     for c1, c2 in zip(val, val[1:]):
#         if c1 == c2:
#             total += 1
#     return total


# double_re = re.compile(r"(?=(.)\1)")


# def count_doubles_regex(val):
#     return len(double_re.findall(val))


val = "".join(random.choice(string.ascii_letters) for i in range(1000000))


# def test_pure_python(benchmark):
#     benchmark(count_doubles, val)


# def test_regex(benchmark):
#     benchmark(count_doubles_regex, val)


# def test_rust(benchmark):  #  <-- Benchmark the Rust version
#     benchmark(myrustlib.count_doubles, val)

if __name__ == "__main__":
    print(myrustlib.big_multiply(122345, 576890))

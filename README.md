# A Rust program to calculate Eulers Number e


## 1: Notes

Pretty much my first/second project in Rust, translated from Java. Very inefficient, ~~500.000 decimals in 10 seconds~~  ~~1.000.000 decimals in 10 seconds~~ ~1.600.000 decimals in 10 seconds, runtime complexity O(n^2). Might improve in the future or not idk

## 2: How To Use

The src folder contains three files:

1. Spigot_Algorithm.rs: Uses an algorithm that calculates e by creating a table-like structure and taking the carry value after each row. Slowest version (~1M digits in 10000s although it ran into stack overflow), but very easy to implement and understand
2. Infinite_Series_Float.rs: Represents data as Floats, decently fast (~1M digits in 11.5s), but huge advantage for converting number to string (merely seconds for 25M digits)
3. Infinite_Series.rs: Represents data as BigInt, fastest algorithm (~1M digits in 4s), but very slow in converting number to string (~1100s for 25M digits)

The best algorithm is Infinite_Series.rs, although conversion from number to string is slow, it's only ~1/3.5 of calculation time which is >3x faster than Floats, hence mmaking it faster overall.

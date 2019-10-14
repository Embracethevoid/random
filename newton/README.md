Small piece of code to calculate the square root of a number.
Using approach Newton iteration

Usage:

cargo run --release < target > < precision >

The number of returned digits is determined by precision

Example:
```
$ cargo run --release 2 50
    Finished release [optimized] target(s) in 0.02s
     Running `target\release\newton.exe 2 50`
1.41421356237309504880168872420969807856967187537694
```
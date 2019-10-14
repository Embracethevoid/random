This is a piece of code to calculate some large fibonacci number(e.g. The 1e6th number in fibonacci array)

Usage:
  cargo run --release targetNumber 
  
Example:
```
$ cargo run --release 50
   Compiling fib v0.1.0 (C:\Users\AshCr\Desktop\random\fib)
    Finished release [optimized] target(s) in 1.05s
     Running `target\release\fib.exe 50`
Finished calculation in 22.9µs
The 50th fibonacci number is "2823960"
```
# rust-rainbow
A rust program to generate a rainbow table based on a dictionary of passwords. This was made to simplify the process of password cracking on hack the box and other pen-testing challenges by pre-compute a rainbow table for rockyou.txt.

## examples:

~~~shell
$ cargo build --release
$ ./target/release/rust-rainbow -w /path/to/rockyou.txt
$ grep PASSWORD-HASH tables/*
~~~

this will make a directory called `tables` with a text file for each sported algorithm. you can then grep for the specific hash across all algorithms if you don't know the algorithm used or the specific table if you do know what algorithm was used to generate the hash.

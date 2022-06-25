# rust-rainbow
A rust program to generate a rainbow table based on a dictionary of passwords. This was made to simplify the process of password cracking on hack the box and other pen-testing challenges by pre-compute a rainbow table for rockyou.txt (or any other password dictionary).

## examples:

~~~shell
$ cargo build --release
$ ./target/release/rust-rainbow -w /path/to/rockyou.txt
$ grep PASSWORD-HASH rockyou_tables/*
~~~

this will make a directory called `rockyou_tables` with a text file for each sported algorithm. you can then grep for the specific hash across all algorithms if you don't know the algorithm used or the specific table if you do know what algorithm was used to generate the hash.

OR

~~~shell
$ cargo build --release
$ ./target/release/rust-rainbow -w /path/to/<WORLIST>
$ grep PASSWORD-HASH <WORDLIST>_tables/*
~~~

## supported algorithms:

- md5
- sha1
- sha256
- sha512
- ntlm_v2

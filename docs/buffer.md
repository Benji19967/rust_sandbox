# Buffer

Why use a buffer when reading from a file?

## Example: reading from a file line by line

If you `read` every line, Rust will need to make a system call on every read to
access the data from Disk. 

A BufReader will perform large, infrequent reads and store the data in memory in a
buffer.

Finally, reading the entire file into memory at once could use up a lot of memory if 
the file is large.

## System call

A program needs to make a system call to request a service from the operating system.

Examples of such services:

- Read from or write to a hard disk drive
- Creation and execution of new processes
- Communication with process scheduler

Example system calls:

- `open`, `read,` `write,` `close`, `wait`, `exec`, `fork`, `exit`, and `kill`

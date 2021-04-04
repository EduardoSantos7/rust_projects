Port Sniffer CLI
------------------------

>A port sniffer is an application designed to probe a server or host for open ports. Such an application may be used by administrators to verify security policies of their networks and by attackers to identify network services running on a host and exploit vulnerabilities.


This is a Rust command line app using only the standard library

How does it work?
----------------

The app could recive the next parameters:

- `-j` to indicate the number of threads to use.

- `-h --help` to show a help message.

- `ip` The host ip.

**Note:** *If the param `-j` is not recived the app will use 4 as default.*

Each thread will scan a port in a range of `0` to `65535`. The scan process consist in create a TcpStream to connect to a port, if the connection succed then it will print a `.` and return the port number.




Run
-----------

```rust
cargo run -- -j 100 127.0.0.1
```

Output example
-------------

```
.........................
3306 is open
5432 is open
9000 is open
.
.
.
```
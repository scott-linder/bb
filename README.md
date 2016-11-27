bb
==

A simple bulletin board.

Rationale
---------

An opportunity to learn Rust and some of the web related crates.

Setup
-----

Create the SQL database by running `sql/create.sql`, and optionally load
some test data with `sql/load.sql`.

Run the service by changing to the `bb` directory and executing `cargo run`.

Obtain a copy of Bootstrap and extract it to `static/boostrap`.

Configure your `httpd` to serve files from `/static` directly, and to reverse
proxy all other requests over HTTP to the service (the hard-coded default
port is `8080`, which you can find in the `src/main.rs` source file).

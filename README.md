# RSH

RSH is a simple toy HTTP server built with rust.

To start the server using `cargo`:

```bash
cargo run
```

Notice that you will need an instance of `mysql` runing on your localhost at port `3306`.

To run the tests:

```bash
cargo test
```

# Features

- [x] Accept connections using TCP sockets.
- [x] HTTP parsing for `request` and `response`.
- [x] Simple files server on a database.
- [x] Response error handling, adjusting HTTP response codes.
- [ ] Multithreading.
- [ ] Cache.
- [ ] Graceful shutdown.

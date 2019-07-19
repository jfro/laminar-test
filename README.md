# Laminar Reliable Ordered test

Both client & server send each other basic strings every 1s via crossbeam_channel's `select!`

* Server: `cargo run -- -s`
* Client: `cargo run`

## Problems

* Client doesn't receive server's messages except first, they don't show up until ~37s into running

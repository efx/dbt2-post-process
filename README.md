# dbt2 post-processing benchmarking

Inspired by Mark Wong's [blog post](https://www.2ndquadrant.com/en/blog/adventure-in-programming-languages-and-simple-statistics/).
See @mw2q's [gist implementations](https://gist.github.com/mw2q).

This is a weekend spike in Rust from a novice. So, contributions welcome!

# running
You must unpack the `mix.log` to your computer. You would also need the Rust toolchain to try this out. ([rustup](https://rustup.rs/) makes that easy).
To execute the binary, use `cargo run`:

```sh
dbt2-post-process> cargo run --release -- ../mix.log 
   Compiling dbt2-post-process v0.1.0 (/home/eli/rust/mark-wong-dbt2-post/dbt2-post-process)
    Finished release [optimized] target(s) in 0.86s
     Running `target/release/dbt2-post-process ../mix.log`
256 second(s) ramping up
``` 

I use the `--release` flag because we are interested in real world performance here.


## observations

This is incomplete so will suspend judgment. But here is a sample generated from [hyperfine](https://github.com/sharkdp/hyperfine):

| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `cargo run --release ../mix.log` | 2.135 ± 0.018 | 2.096 | 2.164 | 1.00 |


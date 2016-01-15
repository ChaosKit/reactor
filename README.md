# reactor

Basic Rust implementation of a modified version of the [Fractal Flame](https://en.wikipedia.org/wiki/Fractal_flame) algorithm.

## Dependencies

- [Rust](https://www.rust-lang.org/) 1.4+
- [Cap'n Proto](https://capnproto.org/) 0.5+

## Usage

Building:

```
cargo build --release
```

Running:

```
./target/release/reactor < testinput.chaos | ./target/release/reactor-client
```

You can inspect the test input with Cap'n Proto's `capnp` tool:

```
capnp decode src/chaoskit.capnp Message < testinput.chaos
```

## Configuration

The following environment variables can be set:

- `PARTICLE_COUNT` — number of particles that are simultaneously "alive", default 10000
- `ITERATION_COUNT` — number of steps to calculate for each particle, default 1000
- `PARTICLE_BUFFER_SIZE` — size of the particle buffer that's sent across threads, default 1000
- `CHANNEL_SIZE` — how many particle buffers can be enqueued, default 10

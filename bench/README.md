# Benchmark
I used this crate to benchmark multiple implementations of my game of life.
It uses the “pulsar” pattern of the game of life and update it 1000 times.

## First version
The first benchmark was made from the commit 1341e2524898a20f46939e128e6490377d7a5fa6 :
```
test tests::bench       ... bench:   6,517,132 ns/iter (+/- 145,113)
test tests::large_bench ... bench:  43,148,900 ns/iter (+/- 1,886,130)
```

## Second version
We parallelized the generation of the new grid with rayon.
It look like spawning thread slow down the computation a lot.
But on a large enough grid we get better result than the sequential version.

```
test tests::bench       ... bench:  17,140,299 ns/iter (+/- 1,994,264)
test tests::large_bench ... bench:  30,443,625 ns/iter (+/- 3,033,592)

```

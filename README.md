# Inventory

[![LANGUAGE](https://img.shields.io/badge/Language-Rust-dea584)](https://www.rust-lang.org/)
[![LICENSE](https://img.shields.io/badge/license-MIT-blue)](https://github.com/gengteng/inventory/blob/main/LICENSE)
![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/gengteng/inventory)
[![dependency status](https://deps.rs/repo/github/gengteng/inventory/status.svg)](https://deps.rs/repo/github/gengteng/inventory)
[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/gengteng/inventory/Rust)](https://github.com/gengteng/inventory/actions/workflows/rust.yml)

This is a redis module that provides a type for inventory deduction in flash sales.

```rust
#[repr(C)]
struct Inventory {
    total: u32,
    current: u32,
}

use std::mem::size_of;
assert_eq!(size_of::<Inventory>(), size_of::<u64>());
```

## Commands

* `inv.set key total`: Set the inventory value of a key.
* `inv.setnx key total`: Set the inventory value of a key, only if the key does not exist.
* `inv.get key`: Get the inventory of a key, including `total` and `current`.
* `inv.ddct key [amount(default=1)]`: Check and deduct inventory. 
* `inv.incr key amount`: Increase `total` and `current`. 
* `inv.return key amount`: Increase `current`.
* `inv.del key`: Delete an inventory.

## Build

Make sure you have Rust installed: https://www.rust-lang.org/tools/install

Then, build as usual:

```shell
cargo build --release
```

When running the tests, you need to explicitly specify the test feature to disable use of the Redis memory allocator when testing:

```shell
cargo test --features test
```

If you forget to do this, you'll see an error mentioning signal: 4, SIGILL: illegal instruction.

## Run

### Linux

```shell
redis-server --loadmodule ./target/release/libinventory.so
```

### Mac OS

```shell
redis-server --loadmodule ./target/release/libinventory.dylib
```

## Benchmark

The following benchmark results are from my new Mac (special thanks to my girlfriend), using the M1 chip. 

### PING

Script:

```shell
$ redis-benchmark -n 10000000 ping
```

Output:
```
  throughput summary: 206752.53 requests per second
  latency summary (msec):
          avg       min       p50       p95       p99       max
        0.138     0.048     0.135     0.183     0.279     2.351
```

<details>
<summary>details</summary>
<pre>
====== ping ======                                                     
  10000000 requests completed in 48.37 seconds
  50 parallel clients
  14 bytes payload
  keep alive: 1
  host configuration "save": 3600 1 300 100 60 10000
  host configuration "appendonly": no
  multi-thread: no

Latency by percentile distribution:
0.000% <= 0.055 milliseconds (cumulative count 2)
50.000% <= 0.135 milliseconds (cumulative count 5688196)
75.000% <= 0.151 milliseconds (cumulative count 8010099)
87.500% <= 0.167 milliseconds (cumulative count 9159920)
93.750% <= 0.175 milliseconds (cumulative count 9433296)
96.875% <= 0.199 milliseconds (cumulative count 9716460)
98.438% <= 0.255 milliseconds (cumulative count 9848161)
99.219% <= 0.287 milliseconds (cumulative count 9935164)
99.609% <= 0.311 milliseconds (cumulative count 9966971)
99.805% <= 0.335 milliseconds (cumulative count 9981121)
99.902% <= 0.375 milliseconds (cumulative count 9991420)
99.951% <= 0.415 milliseconds (cumulative count 9995520)
99.976% <= 0.471 milliseconds (cumulative count 9997690)
99.988% <= 0.551 milliseconds (cumulative count 9998801)
99.994% <= 0.983 milliseconds (cumulative count 9999391)
99.997% <= 1.815 milliseconds (cumulative count 9999717)
99.998% <= 1.879 milliseconds (cumulative count 9999858)
99.999% <= 1.927 milliseconds (cumulative count 9999928)
100.000% <= 2.015 milliseconds (cumulative count 9999963)
100.000% <= 2.135 milliseconds (cumulative count 9999981)
100.000% <= 2.199 milliseconds (cumulative count 9999991)
100.000% <= 2.247 milliseconds (cumulative count 9999996)
100.000% <= 2.311 milliseconds (cumulative count 9999998)
100.000% <= 2.319 milliseconds (cumulative count 9999999)
100.000% <= 2.351 milliseconds (cumulative count 10000000)
100.000% <= 2.351 milliseconds (cumulative count 10000000)

Cumulative distribution of latencies:
1.770% <= 0.103 milliseconds (cumulative count 177015)
97.412% <= 0.207 milliseconds (cumulative count 9741165)
99.593% <= 0.303 milliseconds (cumulative count 9959251)
99.950% <= 0.407 milliseconds (cumulative count 9994962)
99.983% <= 0.503 milliseconds (cumulative count 9998252)
99.990% <= 0.607 milliseconds (cumulative count 9999005)
99.991% <= 0.703 milliseconds (cumulative count 9999129)
99.993% <= 0.807 milliseconds (cumulative count 9999279)
99.994% <= 0.903 milliseconds (cumulative count 9999352)
99.994% <= 1.007 milliseconds (cumulative count 9999404)
99.994% <= 1.103 milliseconds (cumulative count 9999444)
99.995% <= 1.207 milliseconds (cumulative count 9999463)
99.995% <= 1.303 milliseconds (cumulative count 9999468)
99.995% <= 1.407 milliseconds (cumulative count 9999471)
99.995% <= 1.503 milliseconds (cumulative count 9999482)
99.995% <= 1.607 milliseconds (cumulative count 9999521)
99.995% <= 1.703 milliseconds (cumulative count 9999547)
99.997% <= 1.807 milliseconds (cumulative count 9999691)
99.999% <= 1.903 milliseconds (cumulative count 9999896)
100.000% <= 2.007 milliseconds (cumulative count 9999961)
100.000% <= 2.103 milliseconds (cumulative count 9999979)
100.000% <= 3.103 milliseconds (cumulative count 10000000)

Summary:
throughput summary: 206752.53 requests per second
latency summary (msec):
avg       min       p50       p95       p99       max
0.138     0.048     0.135     0.183     0.279     2.351
</pre>
</details>

### Lua script

Initialization:

```shell
redis> hmset bench_lua total 10000000 current 10000000
redis> script load 'local counts = redis.call("HMGET", KEYS[1], "total", "current");local total = tonumber(counts[1]);local current = tonumber(counts[2]);local k = tonumber(ARGV[1]); if current > k then redis.call("HINCRBY", KEYS[1], "current", -k); return k; end;return 0'
```

Script:

```shell
$ redis-benchmark -n 10000000 evalsha [scriptsha] 1 bench_lua 1 1
```

Output:
```
  throughput summary: 184145.11 requests per second
  latency summary (msec):
          avg       min       p50       p95       p99       max
        0.224     0.080     0.223     0.327     0.463     2.479
```

<details>
<summary>details</summary>
<pre>
====== evalsha 8d288ce6effb69b0664723d46a0051772621537a 1 bench_lua 1 1 ======
  10000000 requests completed in 54.31 seconds
  50 parallel clients
  100 bytes payload
  keep alive: 1
  host configuration "save": 3600 1 300 100 60 10000
  host configuration "appendonly": no
  multi-thread: no

Latency by percentile distribution:
0.000% <= 0.087 milliseconds (cumulative count 518)
50.000% <= 0.223 milliseconds (cumulative count 5542612)
75.000% <= 0.255 milliseconds (cumulative count 7561675)
87.500% <= 0.295 milliseconds (cumulative count 8933778)
93.750% <= 0.319 milliseconds (cumulative count 9435941)
96.875% <= 0.351 milliseconds (cumulative count 9705820)
98.438% <= 0.423 milliseconds (cumulative count 9852368)
99.219% <= 0.487 milliseconds (cumulative count 9927421)
99.609% <= 0.535 milliseconds (cumulative count 9966579)
99.805% <= 0.559 milliseconds (cumulative count 9981256)
99.902% <= 0.583 milliseconds (cumulative count 9990980)
99.951% <= 0.607 milliseconds (cumulative count 9995904)
99.976% <= 0.631 milliseconds (cumulative count 9997987)
99.988% <= 0.655 milliseconds (cumulative count 9998827)
99.994% <= 1.023 milliseconds (cumulative count 9999393)
99.997% <= 1.671 milliseconds (cumulative count 9999701)
99.998% <= 1.807 milliseconds (cumulative count 9999850)
99.999% <= 1.943 milliseconds (cumulative count 9999924)
100.000% <= 2.063 milliseconds (cumulative count 9999965)
100.000% <= 2.215 milliseconds (cumulative count 9999981)
100.000% <= 2.343 milliseconds (cumulative count 9999992)
100.000% <= 2.391 milliseconds (cumulative count 9999996)
100.000% <= 2.431 milliseconds (cumulative count 9999998)
100.000% <= 2.447 milliseconds (cumulative count 9999999)
100.000% <= 2.479 milliseconds (cumulative count 10000000)
100.000% <= 2.479 milliseconds (cumulative count 10000000)

Cumulative distribution of latencies:
0.321% <= 0.103 milliseconds (cumulative count 32072)
43.738% <= 0.207 milliseconds (cumulative count 4373816)
91.282% <= 0.303 milliseconds (cumulative count 9128228)
98.285% <= 0.407 milliseconds (cumulative count 9828487)
99.418% <= 0.503 milliseconds (cumulative count 9941792)
99.959% <= 0.607 milliseconds (cumulative count 9995904)
99.992% <= 0.703 milliseconds (cumulative count 9999245)
99.993% <= 0.807 milliseconds (cumulative count 9999315)
99.993% <= 0.903 milliseconds (cumulative count 9999340)
99.994% <= 1.007 milliseconds (cumulative count 9999372)
99.994% <= 1.103 milliseconds (cumulative count 9999420)
99.994% <= 1.207 milliseconds (cumulative count 9999449)
99.995% <= 1.303 milliseconds (cumulative count 9999479)
99.995% <= 1.407 milliseconds (cumulative count 9999524)
99.996% <= 1.503 milliseconds (cumulative count 9999564)
99.996% <= 1.607 milliseconds (cumulative count 9999615)
99.997% <= 1.703 milliseconds (cumulative count 9999746)
99.999% <= 1.807 milliseconds (cumulative count 9999850)
99.999% <= 1.903 milliseconds (cumulative count 9999909)
99.999% <= 2.007 milliseconds (cumulative count 9999948)
100.000% <= 2.103 milliseconds (cumulative count 9999971)
100.000% <= 3.103 milliseconds (cumulative count 10000000)

Summary:
throughput summary: 184145.11 requests per second
latency summary (msec):
avg       min       p50       p95       p99       max
0.224     0.080     0.223     0.327     0.463     2.479
</pre>
</details>

### Inventory module

Initialization:

```shell
redis> inv.set bench_inv 10000000
```

Script:
```shell
$ redis-benchmark -n 10000000 inv.ddct bench_inv
```

Output:
```
  throughput summary: 200553.53 requests per second
  latency summary (msec):
          avg       min       p50       p95       p99       max
        0.155     0.056     0.143     0.247     0.423     2.583
```
<details>
<summary>details</summary>
<pre>
====== inv.ddct bench_inv ======
  10000000 requests completed in 49.86 seconds
  50 parallel clients
  33 bytes payload
  keep alive: 1
  host configuration "save": 3600 1 300 100 60 10000
  host configuration "appendonly": no
  multi-thread: no

Latency by percentile distribution:
0.000% <= 0.063 milliseconds (cumulative count 1)
50.000% <= 0.143 milliseconds (cumulative count 5174947)
75.000% <= 0.167 milliseconds (cumulative count 8039536)
87.500% <= 0.183 milliseconds (cumulative count 8880992)
93.750% <= 0.215 milliseconds (cumulative count 9392336)
96.875% <= 0.295 milliseconds (cumulative count 9694479)
98.438% <= 0.375 milliseconds (cumulative count 9845786)
99.219% <= 0.439 milliseconds (cumulative count 9923638)
99.609% <= 0.495 milliseconds (cumulative count 9964499)
99.805% <= 0.535 milliseconds (cumulative count 9983634)
99.902% <= 0.559 milliseconds (cumulative count 9991677)
99.951% <= 0.583 milliseconds (cumulative count 9996100)
99.976% <= 0.607 milliseconds (cumulative count 9997969)
99.988% <= 0.647 milliseconds (cumulative count 9998834)
99.994% <= 1.079 milliseconds (cumulative count 9999391)
99.997% <= 1.823 milliseconds (cumulative count 9999696)
99.998% <= 2.143 milliseconds (cumulative count 9999849)
99.999% <= 2.247 milliseconds (cumulative count 9999927)
100.000% <= 2.407 milliseconds (cumulative count 9999964)
100.000% <= 2.463 milliseconds (cumulative count 9999983)
100.000% <= 2.495 milliseconds (cumulative count 9999995)
100.000% <= 2.511 milliseconds (cumulative count 9999996)
100.000% <= 2.575 milliseconds (cumulative count 9999998)
100.000% <= 2.583 milliseconds (cumulative count 10000000)
100.000% <= 2.583 milliseconds (cumulative count 10000000)

Cumulative distribution of latencies:
0.849% <= 0.103 milliseconds (cumulative count 84944)
93.381% <= 0.207 milliseconds (cumulative count 9338065)
97.173% <= 0.303 milliseconds (cumulative count 9717262)
98.887% <= 0.407 milliseconds (cumulative count 9888745)
99.688% <= 0.503 milliseconds (cumulative count 9968847)
99.980% <= 0.607 milliseconds (cumulative count 9997969)
99.990% <= 0.703 milliseconds (cumulative count 9998960)
99.991% <= 0.807 milliseconds (cumulative count 9999099)
99.992% <= 0.903 milliseconds (cumulative count 9999223)
99.993% <= 1.007 milliseconds (cumulative count 9999326)
99.994% <= 1.103 milliseconds (cumulative count 9999416)
99.995% <= 1.207 milliseconds (cumulative count 9999468)
99.995% <= 1.303 milliseconds (cumulative count 9999484)
99.995% <= 1.407 milliseconds (cumulative count 9999503)
99.995% <= 1.503 milliseconds (cumulative count 9999526)
99.996% <= 1.607 milliseconds (cumulative count 9999600)
99.996% <= 1.703 milliseconds (cumulative count 9999630)
99.997% <= 1.807 milliseconds (cumulative count 9999677)
99.997% <= 1.903 milliseconds (cumulative count 9999747)
99.998% <= 2.007 milliseconds (cumulative count 9999820)
99.998% <= 2.103 milliseconds (cumulative count 9999841)
100.000% <= 3.103 milliseconds (cumulative count 10000000)

Summary:
throughput summary: 200553.53 requests per second
latency summary (msec):
avg       min       p50       p95       p99       max
0.155     0.056     0.143     0.247     0.423     2.583
</pre>
</details>

### Conclusion

The performance of this module is only slightly better than using lua scripts, perhaps only more complex in-memory data structures are necessary to use the redis module, especially those that are more efficient after deserialization, such as [JSON](https://github.com/RedisJSON/RedisJSON).

## License

MIT

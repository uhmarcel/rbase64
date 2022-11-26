# rbase64
A fast multi-threaded base64 encoding / decoding library and CLI tool, made in Rust.

[![Crates.io](https://img.shields.io/crates/v/rbase64?style=flat-square)](https://crates.io/crates/rbase64)
[![Crates.io](https://img.shields.io/crates/d/rbase64?style=flat-square)](https://crates.io/crates/rbase64)
[![Build Status](https://img.shields.io/github/workflow/status/uhmarcel/rbase64/CI/main?style=flat-square)](https://github.com/uhmarcel/rbase64/actions/workflows/ci.yml?query=branch%3Amain)
[![Coverage Status](https://coveralls.io/repos/github/uhmarcel/rbase64/badge.svg?branch=main)](https://coveralls.io/github/uhmarcel/rbase64?branch=main)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](LICENSE-APACHE)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE-MIT)

---

Have you ever wanted to base64 encode tens of gigabytes of data, but felt your encoder was not fast enough? Me neither.
Either way,
rbase64 is up to the task.


The goal of rbase64 is to provide a fast implementation of base64 encoding. The library is thoroughly tested and optimized
for high throughput. As of v2.0.2, rbase64 is able to achieve up to 11.019 GiB/s encoding and 11.979 GiB/s decoding rates (measured
in MacBook Air M1 2020 laptop, 10~20 MiB batches).

The project also comes with a command line interface powered by the library encoder. While limited by I/O,
it still achieves high performance compared to other alternatives (for example, GNU base64 by a factor of 11x).

For details, see [Performance](#performance).

# Usage
## Library
Add rbase64 to your Cargo.toml dependencies:
```toml
[dependencies]
rbase64 = "2.0.2"
```

**Sample usage:**
```rust
use rbase64;

fn main() {
    let a = b"Hello world";
    let b = "SGVsbG8gd29ybGQ=";

    assert_eq!(rbase64::encode(a), b);
    assert_eq!(&rbase64::decode(b).unwrap(), a);
}
```

## CLI
**Installation:**
```sh
$ cargo install rbase64 --features cli
```


**Usage**: rbase64 [OPTIONS]

Options                      | Description
---------------------------- | ----------------------------
**-d, --decode**             | Decode input (default: false)
**-i, --input \<INPUT\>**    | Input file (stdin if missing)
**-o, --output \<OUTPUT\>**  | Output file (stdout if missing)
**-h, --help**               | Print help information
**-V, --version**            | Print version information

Rbase64 CLI is designed to have the same API as classic GNU base64, however, **it does not append
newline characters to file or stdout output**.

**Basic usage:**
```sh
$ echo -n "Hello world" | rbase64
SGVsbG8gd29ybGQ=
$ echo -n "SGVsbG8gd29ybGQ=" | rbase64 --decode
Hello world
```
# Performance

## Library
Performance is measured using benchmarks with Criterion. Each bench measures encoding/decoding
speed and throughput at varying input sizes in bytes. Measurements below were taken by running
benchmarks on a MacBook Air M1 2020 laptop. Results are compared against some of the top base64 
encoding libraries in [cargo.io](https://crates.io/search?q=base64&sort=downloads).

### Encoding
| Bench (type / bytes) | *rbase64 (v2.0.2)* | [base64 (v0.20.0)](https://github.com/marshallpierce/rust-base64) | [data-encoding (v2.3.2)](https://github.com/ia0/data-encoding) | [rustc-serialize (v0.3.24)](https://github.com/rust-lang/rustc-serialize) | [base64-simd (v0.7.0)](https://github.com/Nugine/simd/tree/main/crates/base64-simd) |
|-----------------|--------------|--------------|--------------|--------------|--------------|
| encode/3        | 188.87 MiB/s | 77.240 MiB/s | 77.208 MiB/s | 91.615 MiB/s | 357.45 MiB/s |
| encode/50       | 1.3597 GiB/s | 904.74 MiB/s | 882.87 MiB/s | 806.29 MiB/s | 1.6693 GiB/s |
| encode/100      | 1.7865 GiB/s | 1.2844 GiB/s | 1.3009 GiB/s | 1.0334 GiB/s | 2.3903 GiB/s |
| encode/500      | 2.6030 GiB/s | 1.8897 GiB/s | 1.9288 GiB/s | 1.2626 GiB/s | 2.8986 GiB/s |
| encode/3072     | 2.7916 GiB/s | 2.3780 GiB/s | 2.4211 GiB/s | 1.4501 GiB/s | 3.1044 GiB/s |
| encode/51200    | 2.8368 GiB/s | 2.4486 GiB/s | 2.4678 GiB/s | 1.4765 GiB/s | 3.1257 GiB/s |
| encode/102400   | 2.8062 GiB/s | 2.4247 GiB/s | 2.4756 GiB/s | 1.4799 GiB/s | 3.0012 GiB/s |
| encode/512000   | 5.0433 GiB/s | 2.5105 GiB/s | 2.5174 GiB/s | 1.4623 GiB/s | 3.0210 GiB/s |
| encode/1048576  | 7.3092 GiB/s | 2.5115 GiB/s | 2.5182 GiB/s | 1.4506 GiB/s | 3.0005 GiB/s |
| encode/5242880  | 9.6995 GiB/s | 2.1806 GiB/s | 2.2009 GiB/s | 1.3416 GiB/s | 2.9529 GiB/s |
| encode/10485760 | 11.019 GiB/s | 2.0549 GiB/s | 2.1037 GiB/s | 1.3438 GiB/s | 2.9763 GiB/s |
| encode/20971520 | 10.702 GiB/s | 2.0814 GiB/s | 2.0638 GiB/s | 1.3138 GiB/s | 2.9818 GiB/s |

### Decoding
| Bench (type / bytes) | *rbase64 (v2.0.2)* | [base64 (v0.20.0)](https://github.com/marshallpierce/rust-base64) | [data-encoding (v2.3.2)](https://github.com/ia0/data-encoding) | [rustc-serialize (v0.3.24)](https://github.com/rust-lang/rustc-serialize) | [base64-simd (v0.7.0)](https://github.com/Nugine/simd/tree/main/crates/base64-simd) |
|-----------------|--------------|--------------|--------------|--------------|--------------|
| decode/3        | 235.55 MiB/s | 52.127 MiB/s | 55.164 MiB/s | 96.523 MiB/s | 308.45 MiB/s |
| decode/50       | 1.7775 GiB/s | 540.87 MiB/s | 728.58 MiB/s | 550.51 MiB/s | 2.1791 GiB/s |
| decode/100      | 2.3152 GiB/s | 942.93 MiB/s | 1.1012 GiB/s | 579.12 MiB/s | 2.8483 GiB/s |
| decode/500      | 2.8945 GiB/s | 1.8286 GiB/s | 1.7942 GiB/s | 563.76 MiB/s | 3.4155 GiB/s |
| decode/3072     | 3.0656 GiB/s | 2.9708 GiB/s | 2.2418 GiB/s | 609.78 MiB/s | 3.6498 GiB/s |
| decode/51200    | 3.1528 GiB/s | 3.0924 GiB/s | 2.3634 GiB/s | 618.20 MiB/s | 3.7184 GiB/s |
| decode/102400   | 3.1314 GiB/s | 3.0878 GiB/s | 2.4015 GiB/s | 618.18 MiB/s | 3.6789 GiB/s |
| decode/512000   | 6.4574 GiB/s | 3.1235 GiB/s | 2.4229 GiB/s | 618.47 MiB/s | 3.7311 GiB/s |
| decode/1048576  | 8.6202 GiB/s | 3.1195 GiB/s | 2.4073 GiB/s | 617.00 MiB/s | 3.7166 GiB/s |
| decode/5242880  | 10.410 GiB/s | 2.8267 GiB/s | 2.2475 GiB/s | 600.88 MiB/s | 3.6338 GiB/s |
| decode/10485760 | 11.257 GiB/s | 2.7416 GiB/s | 2.2384 GiB/s | 599.09 MiB/s | 3.6330 GiB/s |
| decode/20971520 | 11.979 GiB/s | 2.6512 GiB/s | 2.1692 GiB/s | 596.77 MiB/s | 3.6614 GiB/s |

## CLI
Manual benchmarks using randomized binary files. Compared against classic
GNU base64, and race64 (C).

<details>
    <summary>Test 1GiB random bytes</summary>

- GNU base64
```sh
$ time (cat random-1gb.bin | base64 | pv -a > /dev/null)
[ 164MiB/s]
( cat random-1gb.bin | base64 | pv -a > /dev/null; )  7.89s user 0.96s system 106% cpu 8.317 total

$ time (cat random-1gb.b64 | base64 --decode | pv -a > /dev/null)
[ 105MiB/s]
( cat random-1gb.b64 | base64 --decode | pv -a > /dev/null; )  9.16s user 1.01s system 104% cpu 9.699 total
```

- race64 (C)
```sh
$ time (cat random-1gb.bin | ./race64 | pv -a > /dev/null)
[ 898MiB/s]
( cat random-1gb.bin | ./race64 | pv -a > /dev/null; )  0.88s user 1.14s system 128% cpu 1.566 total

$ time (cat random-1gb.b64 | ./race64 -d | pv -a > /dev/null)
[ 723MiB/s]
( cat random-1gb.b64 | ./race64 -d | pv -a > /dev/null; )  0.87s user 0.95s system 127% cpu 1.426 total
```

- ***rbase64***
```sh
$ time (cat random-1gb.bin | rbase64 | pv -a > /dev/null)
[1.71GiB/s]
( cat random-1gb.bin | rbase64 | pv -a > /dev/null; )  0.56s user 0.40s system 121% cpu 0.788 total

$ time (cat random-1gb.b64 | rbase64 --decode | pv -a > /dev/null)
[1.16GiB/s]
( cat random-1gb.b64 | rbase64 --decode | pv -a > /dev/null; )  0.59s user 0.46s system 119% cpu 0.871 total
```
</details>


<details>
    <summary>Test 10GiB random bytes</summary>

- GNU base64
```sh
$ time (cat random-10gb.bin | base64 | pv -a > /dev/null)
[ 154MiB/s]
( cat random-10gb.bin | base64 | pv -a > /dev/null; )  78.74s user 10.96s system 101% cpu 1:28.34 total

$ time (cat random-10gb.b64 | base64 --decode | pv -a > /dev/null)
[ 107MiB/s]
( cat random-10gb.b64 | base64 --decode | pv -a > /dev/null; )  91.02s user 10.00s system 105% cpu 1:35.39 total

```

- race64 (C)
```sh
$ time (cat random-10gb.bin | ./race64 | pv -a > /dev/null)
[ 818MiB/s]
( cat random-10gb.bin | ./race64 | pv -a > /dev/null; )  8.56s user 13.17s system 128% cpu 16.917 total

$ time (cat random-10gb.b64 | ./race64 -d | pv -a > /dev/null)
[ 724MiB/s]
( cat random-10gb.b64 | ./race64 -d | pv -a > /dev/null; )  8.42s user 9.05s system 123% cpu 14.152 total
```

- ***rbase64***
```sh
$ time (cat random-10gb.bin | rbase64 | pv -a > /dev/null)
[1.64GiB/s]
( cat random-10gb.bin | rbase64 | pv -a > /dev/null; )  5.30s user 4.22s system 117% cpu 8.125 total

$ time (cat random-10gb.b64 | rbase64 --decode | pv -a > /dev/null)
[1.18GiB/s]
( cat random-10gb.b64 | rbase64 --decode | pv -a > /dev/null; )  5.58s user 4.40s system 117% cpu 8.491 total
```
</details>

For reference, here's the throughput of piping a file via ```cat``` with no processing:
```sh
$ time (cat random-10gb.b64 | pv -a > /dev/null)
[2.31GiB/s]
( cat random-10gb.b64 | pv -a > /dev/null; )  0.88s user 4.72s system 95% cpu 5.858 total
```

# License
This project is dual-licensed under MIT and Apache 2.0.
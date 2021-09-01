# urio

urio is a [io_uring](https://kernel.dk/io_uring.pdf) library written in Rust. It provides a **safe** Rust-friendly interface.

[![Documentation][docs-badge]][docs-url]
[![Crates.io][crates-badge]][crates-url]
[![Repository][repo-badge]][repo-url]
[![License][license-badge]][license-url]

## Getting started

**NOTE**: Please check your [kernel version](#kernel-support) before you dive in.

You can start using urio by first adding it to your `Cargo.toml`:

```toml
[dependencies]
urio = "0.1"
```

Then, on your `main.rs`:

```rust
use std::os::unix::io::AsRawFd;
use std::{error::Error, fs::File, io::IoSlice};

use urio::Uring;

fn main() -> Result<(), Box<dyn Error>> {
    let mut ring = Uring::new(8)?;
    let file = File::create("hello.txt")?;

    let message = b"Hello, urio!";
    ring.alloc_sqe()?
        .packup_write_vectored(file.as_raw_fd(), &[IoSlice::new(message)], 0);
    ring.submit_and_wait(1)?;

    let cqe = ring.reap_cqe()?;
    let n = cqe.result()?;
    assert_eq!(n, message.len() as _);

    Ok(())
}
```

## Kernel Support

io_uring is available since Linux kernel 5.1. So urio requires at least kernel 5.1 or newer. Even your kernel supports io_uring, some new features may be not supported. Please check [API Docs][docs-url] and make sure that features you want to use is supported by the kernel you are using.

[docs-badge]: https://docs.rs/urio/badge.svg
[docs-url]: https://docs.rs/urio
[crates-badge]: https://img.shields.io/crates/v/urio.svg?logo=rust
[crates-url]: https://crates.io/crates/urio
[repo-badge]: https://img.shields.io/badge/Repository-urio-77CCBB.svg?logo=github
[repo-url]: https://github.com/seunghunee/urio
[license-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[license-url]: ./LICENSE

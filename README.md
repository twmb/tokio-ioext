tokio-try-read-full
===================

 [![Crates.io](https://img.shields.io/crates/v/tokio-try-read-full.svg)](https://crates.io/crates/tokio-try-read-full) [![Documentation](https://docs.rs/tokio-try-read-full/badge.svg)](https://docs.rs/tokio-try-read-full/)

This crate complemnts `tokio-io`'s `read_exact` for scenarios where you want to
read as many bytes as possible into a buffer until either the buffer is full or
the reader returns `EOF`.

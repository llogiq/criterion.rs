extern crate criterion;

use std::mem;

use criterion::Criterion;

const SIZE: usize = 1024 * 1024;

#[test]
fn dealloc() {
    Criterion::default()
        .bench_function("dealloc", |b| {
            b.iter_with_setup(|| (0..SIZE).map(|_| 0u8).collect::<Vec<_>>(), |v| mem::drop(v))
        });
}

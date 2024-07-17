// MIT License
//
// Copyright (c) 2024 Nathan H. Keough
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use can_types::{conversion::Conversion, protocol::j1939::identifier::IdJ1939};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn id_to_bits(id: &IdJ1939) {
    let id_bits = id.into_bits();
    assert_eq!(0b000_011_0_0_11110000_00000100_00000000, id_bits);
}

#[cfg(feature = "alloc")]
fn id_to_hex(id: &IdJ1939) {
    let id_hex = id.into_hex();
    assert_eq!("0CF00400", id_hex)
}

fn id_from_bits(bits: u32) {
    let id = IdJ1939::from_bits(bits);
    assert_eq!(0b000_011_0_0_11110000_00000100_00000000, id.into_bits());
}

fn id_from_hex(hex_str: &str) {
    let id = IdJ1939::from_hex(hex_str);
    assert_eq!(0b000_011_0_0_11110000_00000100_00000000, id.into_bits());
}

pub fn id_bench(c: &mut Criterion) {
    let id_bits: u32 = 217056256;
    let id_hex = "0CF00400";
    let id = IdJ1939::from_bits(id_bits);

    let mut group = c.benchmark_group("id");
    group.throughput(criterion::Throughput::Elements(1));
    group.bench_function("to_bits", |b| b.iter(|| black_box(id_to_bits(&id))));

    #[cfg(feature = "alloc")]
    group.bench_function("to_hex", |b| b.iter(|| black_box(id_to_hex(&id))));

    group.bench_function("from_bits", |b| b.iter(|| black_box(id_from_bits(id_bits))));
    group.bench_function("from_hex", |b| b.iter(|| black_box(id_from_hex(id_hex))));
    group.finish();
}

criterion_group!(benches, id_bench,);

criterion_main!(benches);

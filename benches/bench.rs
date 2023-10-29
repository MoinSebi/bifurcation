use criterion::{criterion_group, criterion_main, Criterion};
use bifurcation::{bifurcation_analysis_btree, bifurcation_analysis_bheap, bifurcation_analysis_meta, bifurcation_analysis_sort, sort_array_vec};
use bifurcation::test::{data_creation, load_data};


/// Bench with criterion
/// Checking two data sets and two approaches.
fn criterion_benchmark(c: &mut Criterion) {
    // Create the data
    let mut data = data_creation(500000, 100, 5000);
    let mut data2 = load_data("data/test.index.20000.txt");

    // Sort the data
    sort_array_vec(&mut data2);
    sort_array_vec(&mut data);

    // Report length
    println!("The data is of length {} {}", data.len(), data2.len());

    // Run version 1
    c.bench_function("Bifurcation neu", |b| b.iter(|| bifurcation_analysis_meta(&data2[..])));
    c.bench_function("Bifurcation new", |b| b.iter(|| bifurcation_analysis_meta(&data[..])));

    // Run version
    c.bench_function("Bifurcation sort", |b| b.iter(|| bifurcation_analysis_sort(&data2[..])));
    c.bench_function("Bifurcation sort", |b| b.iter(|| bifurcation_analysis_sort(&data[..])));

    c.bench_function("Bifurcation btree", |b| b.iter(|| bifurcation_analysis_btree(&data2[..])));
    c.bench_function("Bifurcation btree", |b| b.iter(|| bifurcation_analysis_btree(&data[..])));

    c.bench_function("Bifurcation bheap", |b| b.iter(|| bifurcation_analysis_bheap(&data2[..])));
    c.bench_function("Bifurcation bheap", |b| b.iter(|| bifurcation_analysis_bheap(&data[..])));
}

// Run the bench
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
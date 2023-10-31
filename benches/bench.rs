use criterion::{criterion_group, criterion_main, Criterion};
use bifurcation::{bifurcation_meta, bifurcation_vec_sorted, sort_array_vec, bifurcation_sort_inplace, bifurcation_sort_hold, bifurcation_btreeset};
use bifurcation::test::{data_creation, load_data};


/// Bench with criterion
/// Checking two data sets and two approaches.
fn criterion_benchmark(c: &mut Criterion) {
    // Create the data
    let mut data_simulated = data_creation(500000, 20, 40000);
    let mut data_real = load_data("data/test.index.20000.txt");

    // Sort the data
    sort_array_vec(&mut data_real);
    sort_array_vec(&mut data_simulated);

    // Report length
    println!("The data is of length {} {}", data_simulated.len(), data_real.len());



    // Meta run
    c.bench_function("Bifurcation meta", |b| b.iter(|| bifurcation_meta(&data_real[..])));
    c.bench_function("Bifurcation meta", |b| b.iter(|| bifurcation_meta(&data_simulated[..])));

    // Sorted (sort function)
    c.bench_function("Bifurcation sort", |b| b.iter(|| bifurcation_vec_sorted(&data_real[..])));
    c.bench_function("Bifurcation sort", |b| b.iter(|| bifurcation_vec_sorted(&data_simulated[..])));

    // Sort inplace
    c.bench_function("Bifurcation sort inplace", |b| b.iter(|| bifurcation_sort_inplace(&data_real[..])));
    c.bench_function("Bifurcation sort inplace", |b| b.iter(|| bifurcation_sort_inplace(&data_simulated[..])));

    // Sort hold
    c.bench_function("Bifurcation sort hold", |b| b.iter(|| bifurcation_sort_hold(&data_real[..])));
    c.bench_function("Bifurcation sort hold", |b| b.iter(|| bifurcation_sort_hold(&data_simulated[..])));

    // Btree
    c.bench_function("Bifurcation btree", |b| b.iter(|| bifurcation_btreeset(&data_real[..])));
    c.bench_function("Bifurcation btree", |b| b.iter(|| bifurcation_btreeset(&data_simulated[..])));
}

// Run the bench
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
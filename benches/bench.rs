use criterion::{criterion_group, criterion_main, Criterion};
use bifurcation::bifurcation_analysis_meta;

/// Ach du heilige SCHEISSE
/// Creates data which should reflect real graph data
pub fn data_creation() -> Vec<[u32; 3]> {
    let mut mm = Vec::new();
    // for x in 0..100{
    //     mm.push([x, 1000000-x, 1]);
    // }

    for x in 100..6000000{
        if x%20 == 0{
            mm.push([x,x+500000,10])
        } else if x%5 == 0{
            mm.push([x+3, x+10000,10])
        } else {
            mm.push([x+1, x+1,10])

        }
    }
    mm.sort();
    mm
}



/// This bench runs the bifurcation_analysis_meta function from lib
pub fn bench_bifurcation_analysis_meta(input: &[[u32; 3]]){
    let _dd = bifurcation_analysis_meta(input);
    //println!("{}", dd.len());
}


/// Bench with criterion
/// I seems like a 1_000_000 vector takes ~ 5 ms on my setting.
fn criterion_benchmark(c: &mut Criterion) {
    let data = data_creation();
    println!("The data is of length {}", data.len());
    c.bench_function("faster network4", |b| b.iter(|| bench_bifurcation_analysis_meta(&data[..])));


}

// Run the bench
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
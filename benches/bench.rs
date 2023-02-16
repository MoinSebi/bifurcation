use criterion::{black_box, criterion_group, criterion_main, Criterion};
use bifurcation::bifurcation_analysis_meta;

pub fn make_index() -> Vec<[u32; 3]> {
    let mut mm = Vec::new();
    mm.push([1,1000000,2]);
    for x in 0..1000000{
        if x%2 == 0{
            mm.push([x,x+5,10])
        } else if x%5 == 0{
            mm.push([x, x-1,10])
        } else {
            mm.push([x, x+2,10])

        }
    }
    mm.sort();
    mm
}

pub fn test(input: &Vec<[u32; 3]>){
    let ff = bifurcation_analysis_meta(&input);

}

fn criterion_benchmark(c: &mut Criterion) {
    let data = make_index();
    c.bench_function("faster network4", |b| b.iter(|| test(&data)));


}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
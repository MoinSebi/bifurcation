use criterion::{black_box, criterion_group, criterion_main, Criterion};
use bifurcation::bifurcation_analysis_meta;

pub fn make_index() -> Vec<[u32; 3]> {
    let mut mm = Vec::new();
    mm.push([1,100000,2]);
    for x in 0..1000{
        for y in (0..1000).rev(){
            mm.push([x,y,x])
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
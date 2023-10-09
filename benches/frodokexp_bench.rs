use criterion::{criterion_group, criterion_main, Criterion};
use frodokexp::{gen_pp, gen_a, gen_b, encaps, decaps, Matrix};

pub fn criterion_benchmark(c: &mut Criterion) {
    //c.bench_function("gen_pp", |b| b.iter(|| gen_pp));
   let seed = gen_pp();
   c.bench_function("gen", |b| b.iter(|| {
       gen_a(seed);
   }));
   c.bench_function("encaps", |b| b.iter_batched(|| -> (Matrix, Matrix)
        {
   let (b_a, (_sk_a, _f_a)) = gen_a(seed);
   let (_b_b, (sk_b, _f_b)) = gen_b(seed);
   (b_a, sk_b)
        },
        |(b,sk)| encaps(b, sk), 
        criterion::BatchSize::SmallInput,
            ));
   c.bench_function("decaps", |b| b.iter_batched(|| -> (Matrix, Matrix, Matrix, u64)
                                                {
                                                    let (b_a, (sk_a, f_a)) = gen_a(seed);
                                                    let (b_b, (sk_b, _f_b)) = gen_b(seed);
                                                    let (_k, ct) = encaps(b_a, sk_b);
                                                    (b_b, sk_a, f_a, ct)
                                                },
                                                |(b,sk, f, ct)| decaps(b, sk, f, ct), 
                                                criterion::BatchSize::SmallInput,
                                                ));
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

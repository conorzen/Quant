use criterion::{black_box, criterion_group, criterion_main, Criterion};
use quant_core::models::derivatives::black_scholes;
use quant_core::types::{OptionsData, OptionType};

fn default_option() -> OptionsData {
    OptionsData {
        spot: 100.0,
        strike: 100.0,
        time: 1.0,
        risk_free_rate: 0.05,
        dividend: 0.02,
        sigma: 0.2,
        option_type: OptionType::Call,
    }
}

fn bench_black_scholes_price(c: &mut Criterion) {
    let option = default_option();
    c.bench_function("black_scholes_price", |b| {
        b.iter(|| black_scholes::price(black_box(&option)))
    });
}

fn bench_black_scholes_10000(c: &mut Criterion) {
    // simulate pricing 1000 options at once
    let options: Vec<OptionsData> = (0..10000)
        .map(|i| OptionsData {
            spot: 100.0 + i as f64 * 0.1,
            ..default_option()
        })
        .collect();

    c.bench_function("black_scholes_10000_options", |b| {
        b.iter(|| {
            options.iter().map(|o| black_scholes::price(black_box(o))).collect::<Vec<_>>()
        })
    });
}

criterion_group!(benches, bench_black_scholes_price, bench_black_scholes_10000);
criterion_main!(benches);
extern crate argon2;
extern crate argon2rs;
#[macro_use]
extern crate criterion;
extern crate jasonus;
extern crate rand;

use criterion::{Criterion, Fun};
use jasonus::config::{default_lanes, DEFAULT_HASH_LENGTH, DEFAULT_ITERATIONS, DEFAULT_MEMORY_SIZE,
                      DEFAULT_SALT_LENGTH};
use rand::rngs::OsRng;
use rand::RngCore;

const PASSWORD: &str = "P@ssw0rd";
const SAMPLE_SIZE: usize = 10;

fn bench_crates(c: &mut Criterion) {
    // argon2rs
    let hasher = argon2rs::Argon2::new(
        /* passes */ DEFAULT_ITERATIONS,
        /* lanes */ default_lanes(),
        /* kib */ DEFAULT_MEMORY_SIZE,
        /* variant */ argon2rs::Variant::Argon2i,
    ).unwrap();
    let argon2rs = Fun::new("argon2rs", move |b, _| {
        b.iter(|| {
            let mut out = [0u8; DEFAULT_HASH_LENGTH as usize];

            let password = PASSWORD.as_bytes();

            let mut rng = OsRng::new().unwrap();
            let mut salt = [0u8; DEFAULT_SALT_LENGTH as usize];
            rng.fill_bytes(&mut salt);

            hasher.hash(
                /* out */ &mut out,
                /* p */ password,
                /* s */ &salt,
                /* k */ &[],
                /* x */ &[],
            );
        });
    });

    // jasonus
    let mut hasher = jasonus::Hasher::default();
    hasher
        .configure_variant(jasonus::config::Variant::Argon2i)
        .configure_password_clearing(false)
        .opt_out_of_secret_key(true);
    let jasonus = Fun::new("jasonus", move |b, _| {
        b.iter(|| {
            let _ = hasher.with_password(PASSWORD).hash_raw().unwrap();
        })
    });

    // rust-argon2
    let config = argon2::Config {
        variant: argon2::Variant::Argon2i,
        version: argon2::Version::Version13,
        mem_cost: DEFAULT_MEMORY_SIZE,
        time_cost: DEFAULT_ITERATIONS,
        lanes: default_lanes(),
        thread_mode: argon2::ThreadMode::Parallel,
        secret: &[],
        ad: &[],
        hash_length: DEFAULT_HASH_LENGTH,
    };
    let rust_argon2 = Fun::new("rust-argon2", move |b, _| {
        b.iter(|| {
            let password = PASSWORD.as_bytes();

            let mut rng = OsRng::new().unwrap();
            let mut salt = [0u8; DEFAULT_SALT_LENGTH as usize];
            rng.fill_bytes(&mut salt);

            let _ = argon2::hash_raw(password, &salt[..], &config).unwrap();
        });
    });

    let functions = vec![argon2rs, jasonus, rust_argon2];
    c.bench_functions("bench_crates", functions, 0);
}

criterion_group!{
    name = benches;
    config = Criterion::default().sample_size(SAMPLE_SIZE);
    targets = bench_crates
}
criterion_main!(benches);

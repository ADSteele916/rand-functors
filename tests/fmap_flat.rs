use std::collections::HashMap;

use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use rand_functors::{Counter, Enumerator, FlattenableRandomStrategy, Functor, Sampler};

fn random_process<S: FlattenableRandomStrategy>(
    rng: &mut (impl Clone + Rng),
    base: u8,
) -> S::Functor<u8> {
    let mut functor = Functor::pure(base);
    functor = S::fmap_rand_range(functor, 0..=16, rng, |d, r: u8| d.saturating_sub(r));
    S::fmap_flat(functor, |d| {
        if d != 0 {
            Functor::pure(d)
        } else {
            let f = Functor::pure(d);
            S::fmap_rand_range(f, 200..=210, rng, |_, r: u8| r)
        }
    })
}

#[test]
fn test_flat_map_sampler() {
    let mut rng = ChaCha8Rng::seed_from_u64(0);
    let output = random_process::<Sampler>(&mut rng, 11);
    assert_eq!(output, 207);
}

#[test]
fn test_flat_map_enumerator() {
    let mut rng = ChaCha8Rng::seed_from_u64(0);
    let output = random_process::<Enumerator>(&mut rng, 9);

    assert_eq!(output.len(), 17 * 11);

    let counts = output.iter().fold(HashMap::new(), |mut map, d| {
        *map.entry(d).or_insert(0usize) += 1;
        map
    });

    for i in 1..=9 {
        assert_eq!(counts[&i], 11);
    }
    for i in 200..=210 {
        assert_eq!(counts[&i], 8);
    }
}

#[test]
fn test_flat_map_counter() {
    let mut rng = ChaCha8Rng::seed_from_u64(0);
    let output = random_process::<Counter>(&mut rng, 9);

    assert_eq!(output.len(), 20);
    assert_eq!(output.values().sum::<usize>(), 17 * 11);

    for i in 1..=9 {
        assert_eq!(output[&i], 11);
    }
    for i in 200..=210 {
        assert_eq!(output[&i], 8);
    }
}

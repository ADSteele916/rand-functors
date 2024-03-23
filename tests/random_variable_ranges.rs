use std::collections::HashMap;

use rand::prelude::*;
use rand::Rng;
use rand_chacha::ChaCha8Rng;
use rand_functors::{Counter, Enumerator, Functor, PopulationSampler, RandomStrategy, Sampler};

fn random_process<S: RandomStrategy>(rng: &mut impl Rng, base: u16) -> S::Functor<u16> {
    let functor = Functor::pure(base);
    S::fmap_rand_range(functor, 217..=255, rng, |d, r: u8| (d * (r as u16)) / 255)
}

#[test]
fn test_rand_range_sampler() {
    let mut rng = ChaCha8Rng::seed_from_u64(0);
    let d = random_process::<Sampler>(&mut rng, 40);
    assert_eq!(d, 37);
}

#[test]
fn test_rand_range_population_sampler() {
    let mut rng = ChaCha8Rng::seed_from_u64(0);
    let d = random_process::<PopulationSampler<20>>(&mut rng, 40);
    assert_eq!(d.len(), 20);
    assert!(d.iter().all(|d| (34..=40).any(|target| target == *d)))
}

#[test]
fn test_rand_range_enumerator() {
    let mut rng = ChaCha8Rng::seed_from_u64(0);
    let d = random_process::<Enumerator>(&mut rng, 40);

    assert_eq!(d.len(), 39);

    let counts = d.iter().fold(HashMap::new(), |mut map, d| {
        *map.entry(d).or_insert(0usize) += 1;
        map
    });
    assert_eq!(counts[&34], 7);
    assert_eq!(counts[&35], 6);
    assert_eq!(counts[&36], 6);
    assert_eq!(counts[&37], 7);
    assert_eq!(counts[&38], 6);
    assert_eq!(counts[&39], 6);
    assert_eq!(counts[&40], 1);
}

#[test]
fn test_rand_range_counter() {
    let mut rng = ChaCha8Rng::seed_from_u64(0);
    let d = random_process::<Counter>(&mut rng, 40);

    assert_eq!(d.len(), 7);
    assert_eq!(d.values().sum::<usize>(), 39);

    assert_eq!(d[&34], 7);
    assert_eq!(d[&35], 6);
    assert_eq!(d[&36], 6);
    assert_eq!(d[&37], 7);
    assert_eq!(d[&38], 6);
    assert_eq!(d[&39], 6);
    assert_eq!(d[&40], 1);
}

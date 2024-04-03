use std::collections::HashMap;

use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use rand_functors::*;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct State {
    a: u16,
    b: [u8; 2],
}

fn random_process<S: RandomStrategy>(rng: &mut impl Rng, mut s: State) -> S::Functor<State> {
    s.a += 2;
    let mut sc = Functor::pure(s);
    sc = S::fmap_rand(sc, rng, |mut s, r| {
        if r {
            s.a -= 1
        }
        s
    });
    sc = S::fmap(sc, |s| State {
        a: s.a.wrapping_sub(1),
        b: s.b,
    });
    sc = S::fmap_rand(sc, rng, |mut s, r| {
        s.b[0] = s.b[0].wrapping_add(r);
        s
    });
    S::fmap_rand(sc, rng, |mut s, r| {
        s.a = s.a.wrapping_add(r);
        s
    })
}

#[test]
fn test_sampler() {
    let mut rng = ChaCha8Rng::seed_from_u64(0);
    let s = State { a: 45, b: [5, 98] };
    let output = random_process::<Sampler>(&mut rng, s);
    assert_eq!(
        output,
        State {
            a: 49252,
            b: [108, 98],
        }
    );
}

#[test]
fn test_population_sampler() {
    const N: usize = 103;

    let mut rng = ChaCha8Rng::seed_from_u64(0);
    let s = State { a: 14, b: [90, 19] };
    let output = random_process::<PopulationSampler<N>>(&mut rng, s);
    assert_eq!(output.len(), N);

    let counts = output.iter().fold(HashMap::new(), |mut map, s| {
        *map.entry(s).or_insert(0) += 1;
        map
    });
    assert!(counts.values().copied().all(|count| count <= 2));

    assert!(output.iter().all(|s| s.b[1] == 19));
}

#[test]
fn test_enumerator() {
    let mut rng = ChaCha8Rng::seed_from_u64(0);
    let s = State { a: 74, b: [0, 47] };
    let output = random_process::<Enumerator>(&mut rng, s);

    assert_eq!(
        output.len(),
        2 * 2_usize.pow(u8::BITS) * 2_usize.pow(u16::BITS)
    );

    let a_counts = output.iter().fold(HashMap::new(), |mut map, s| {
        *map.entry(s.a).or_insert(0) += 1;
        map
    });
    assert_eq!(a_counts.len(), 2_usize.pow(u16::BITS));
    assert!(a_counts
        .values()
        .all(|count| *count == 2 * 2_usize.pow(u8::BITS)));

    let b0_counts = output.iter().fold(HashMap::new(), |mut map, s| {
        *map.entry(s.b[0]).or_insert(0) += 1;
        map
    });
    assert_eq!(b0_counts.len(), 2_usize.pow(u8::BITS));
    assert!(b0_counts
        .values()
        .all(|count| *count == 2 * 2_usize.pow(u16::BITS)));

    assert!(output.iter().all(|s| s.b[1] == 47));
}

#[test]
fn test_counter() {
    let mut rng = ChaCha8Rng::seed_from_u64(0);
    let s = State {
        a: 5286,
        b: [253, 199],
    };
    let output = random_process::<Counter<ahash::RandomState>>(&mut rng, s);

    assert_eq!(output.len(), 2_usize.pow(u8::BITS) * 2_usize.pow(u16::BITS));

    assert!(output.values().copied().all(|count| count == 2));

    let a_counts = output.iter().fold(HashMap::new(), |mut map, (s, count)| {
        *map.entry(s.a).or_insert(0) += count;
        map
    });
    assert_eq!(a_counts.len(), 2_usize.pow(u16::BITS));
    assert!(a_counts
        .values()
        .all(|count| *count == 2 * 2_usize.pow(u8::BITS)));

    let b0_counts = output.iter().fold(HashMap::new(), |mut map, (s, count)| {
        *map.entry(s.b[0]).or_insert(0) += count;
        map
    });
    assert_eq!(b0_counts.len(), 2_usize.pow(u8::BITS));
    assert!(b0_counts
        .values()
        .all(|count| *count == 2 * 2_usize.pow(u16::BITS)));

    assert!(output.iter().all(|(s, _)| s.b[1] == 199));
}

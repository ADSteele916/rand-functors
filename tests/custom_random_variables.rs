use rand::distributions::Standard;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use rand_functors::{Enumerator, Functor, RandomStrategy, RandomVariable};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Pair<A: Clone + RandomVariable, B: Clone + RandomVariable>
where
    Standard: Distribution<A>,
    Standard: Distribution<B>,
{
    x: A,
    y: B,
}

impl<A: Clone + RandomVariable, B: Clone + RandomVariable> Distribution<Pair<A, B>> for Standard
where
    Standard: Distribution<A>,
    Standard: Distribution<B>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Pair<A, B> {
        Pair {
            x: self.sample(rng),
            y: self.sample(rng),
        }
    }
}

impl<A: Clone + RandomVariable, B: Clone + RandomVariable> RandomVariable for Pair<A, B>
where
    Standard: Distribution<A>,
    Standard: Distribution<B>,
{
    fn sample_space() -> impl Iterator<Item = Self> {
        A::sample_space().flat_map(|a| B::sample_space().map(move |b| Pair { x: a.clone(), y: b }))
    }
}

fn dummy_random_process<S: RandomStrategy>(rng: &mut impl Rng) -> S::Functor<Pair<bool, u8>> {
    S::fmap_rand(Functor::pure(()), rng, |_, pair| pair)
}

#[test]
fn test_enumerator_on_custom_random_variable() {
    let mut rng = ChaCha8Rng::seed_from_u64(0);
    let enumeration = dummy_random_process::<Enumerator>(&mut rng);
    assert_eq!(
        enumeration,
        [false, true]
            .into_iter()
            .flat_map(|x| (u8::MIN..=u8::MAX).map(move |y| { Pair { x, y } }))
            .collect::<Vec<_>>()
    );
}

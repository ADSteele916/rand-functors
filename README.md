# rand-functors

`rand-functors` provides an abstraction over different ways of evaluating random processes expressed as functions of both deterministic and stochastic data. This is achieved using a combination of a type-based version of the Strategy pattern and functional programming's Functor pattern.

A motivating problem for this crate is the code duplication present across these two functions modelling the same random process:
```rust
use rand::random;

fn next_state(mut state: u8) -> u8 {
    state = state.wrapping_add(random());
    if random() {
        state %= 3;
    }
    state
}

fn next_states(state: u8) -> Vec<u8> {
    let mut out: Vec<_> = (0..=255).map(|r| state.wrapping_add(r)).collect();
    out.append(&mut out.iter().copied().map(|i| i % 3).collect());
    out
}
```
While these functions may appear different, the same random process is embedded in both of them. A random `u8` is added to `state` and then, if a random `bool` is `true`, the state will be set to itself modulo 3.

This redundant implementation of the random process could pose issues during a refactor. If one decides to change the `%= 3` to a `%= 5` in `next_state`, he or she will need to make the corresponding update in `next_states`.

Using `rand-functors`, these two functions can be combined as:
```rust
use rand::rng;
use rand_functors::{Functor, RandomStrategy};

fn next_state<S: RandomStrategy>(state: u8) -> S::Functor<u8> {
    let mut out = S::fmap_rand(Functor::pure(state), &mut rng(), |s, r| s.wrapping_add(r));
    out = S::fmap_rand(out, &mut rng(), |s, r| if r { s % 3 } else { s });
    out
}
```
This new implementation makes `next_state` generic over a `RandomStrategy` `S`. Its return type is also changed to the `Functor` associated with `S`. Inside, `state` is converted from `u8` to `S::Functor<u8>`. The remainder of the function is essentially the same as the original `next_state`, but each operation a random sample is now wrapped in a call to `S::fmap_rand`. Calling `next_state::<Sampler>(s)` would be equivalent to calling `next_state(s)` before. Similarly, one could call `next_state::<Enumerator>(s)` instead of using `next_states(s)`, which would require maintaining a separate implementation of the same core process.

At present, `rand-functors` only supports random variables that are either of type `bool` or of a numeric type occupying no more than 16 bits by default. However, it is possible to implement all the requisite traits for a custom data type.

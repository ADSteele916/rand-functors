# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.9.0] - 2025-02-11

### Changed

- `rand-functors` now depends on version `0.9` of `rand`.
- `isize` and `usize` no longer implement `RandomVariable`.

## [0.8.0] - 2024-05-16

### Added

- This changelog.

### Changed

- `fmap_flat` is now an associated function of the new trait `FlattenableRandomStrategy`.
- `fmap_flat` no longer takes an `impl Rng` as a a parameter.

## [0.7.0] - 2024-04-24

### Changed

- Made `Counter` generic over unsigned numeric types. This lets users create `Counter` strategies with custom count types for optimization or compatibility purposes.

## [0.6.0] - 2024-04-17

### Changed

- Made `rand-functors` compatible with `no-std` crates by separating allocating `RandomStrategy` implementations into the `alloc` and `std` features.
    - `Sampler` is available without any features.
    - `PopulationSampler` and `Enumerator` are available with the `alloc` or `std` features.
    - `UniqueEnumerator` and `Counter` are available with the `std` feature.

## [0.5.0] - 2024-04-11

### Added

- A new associated function for `RandomStrategy`: `fmap_flat`. `fmap_flat` provides a mechanism for removing one layer of `Functor` nesting and enables new branching behaviour in random processes.
- An integration test for `fmap_flat` which tests a random process with multiple branches.

## [0.4.0] - 2024-04-08

### Added

- New `RandomStrategy`: `UniqueEnumerator`. This strategy produces the set of all possible outputs of a random process, without counting duplicates.

## [0.3.0] - 2024-04-06

### Changed

- `fmap` is now an associated function of `RandomStrategy`, rather than a method of `Functor`. This removes the `Functor::Output` hack, which also prevented chaining `fmap` with `fmap_flat`.

### Fixed

- Type system error that prevented chaining `Functor::fmap` with other operations.

## [0.2.0] - 2024-03-23

### Added

- The `RandomStrategy::fmap_rand_range` associated function and the `RandomRangeVariable` trait, which enable sampling from a range of type `RandomVariable`.
- An integration test for `fmap_rand_range`, which tests a random process involving a multiplication by a number between 217 and 255, inclusive.
- New `Functor` implementation for `HashSet`, to allow downstream crates to define their own implementation of `RandomStrategy` with a `HashSet` as its `Functor`.

### Changed

- All primitive numeric types now implement `RandomVariable`.

### Fixed

- Broken Markdown in `README.md`.

## [0.1.2] - 2024-03-21

### Added

- Further documentation on use cases for `Enumerator` and `Counter`.

### Changed

- All methods in implementations of `Functor`, `RandomStrategy`, and `RandomVariable` now have the `[inline]` attribute, allowing inlining across crate boundaries.

## [0.1.1] - 2024-03-15

### Changed

- Renamed `rand` parameter of `RandomStrategy::fmap_rand`  to `rng`, to prevent naming collisions with the `rand` crate.
- `Counter::fmap_rand` will now pre-allocate enough space on the heap for a `HashMap` with `f.len()` elements, eliminating unnecessary allocations.

## [0.1.0] - 2024-03-11

### Added

- The traits `RandomStrategy`, which governs how to handle the random parts of a stochastic process, and `Functor`, which governs how to store the inputs, outputs, and intermediate results of a `RandomStrategy`.
- The `Inner` trait, which is automatically implemented for types which may be contained within a `Functor`.
- The `RandomVariable` trait, which describes types that are enumerable and can be sampled from uniformly.
- The `Sampler` strategy, which produces a single possible output of a random process.
- The `PopulationSampler` strategy, which creates a `Vec` of at most `N` possible outputs of a random process.
- The `Enumerator` strategy, which creates a `Vec` of all possible outputs of a random process.
- The `Counter` strategy, which creates a `HashMap` of all possible outputs of a random process and counts of their of occurrences.

[unreleased]: https://github.com/ADSteele916/rand-functors/compare/v0.9.0...HEAD
[0.9.0]: https://github.com/ADSteele916/rand-functors/compare/v0.8.0...v0.9.0
[0.8.0]: https://github.com/ADSteele916/rand-functors/compare/v0.7.0...v0.8.0
[0.7.0]: https://github.com/ADSteele916/rand-functors/compare/v0.6.0...v0.7.0
[0.6.0]: https://github.com/ADSteele916/rand-functors/compare/v0.5.0...v0.6.0
[0.5.0]: https://github.com/ADSteele916/rand-functors/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/ADSteele916/rand-functors/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/ADSteele916/rand-functors/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/ADSteele916/rand-functors/compare/v0.1.2...v0.2.0
[0.1.2]: https://github.com/ADSteele916/rand-functors/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/ADSteele916/rand-functors/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/ADSteele916/rand-functors/releases/tag/v0.1.0

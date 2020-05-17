//! Provides a small helper macro for creating iterators out of values without allocation.

#![no_std]
#![cfg_attr(feature = "nightly", feature(array_value_iter))]

/// Creates an `Iterator` containing the arguments, as a [`Chain`][Chain] of [`Once`][Once] iterators.
///
/// The created iterator allows arguments to be moved out of the iterator, and does not allocate.
///
/// This is equivalent to creating a [`Vec`][Vec] and using `into_iter`, but without the heap allocation.
/// Alternatively, this has similar behaviour as the feature [`array_value_iter`](https://github.com/rust-lang/rust/issues/65798),
/// but is permissible in stable Rust, without the need for const generics.
///
/// # Example
/// ```
/// use chained_iter::chained_iter;
/// assert_eq!(vec![1, 2, 3, 4], chained_iter![1, 2, 3, 4].collect::<Vec<_>>());
/// ```
/// [Chain]: https://doc.rust-lang.org/nightly/core/iter/struct.Chain.html
/// [Once]: https://doc.rust-lang.org/nightly/core/iter/struct.Once.html
/// [Vec]: https://doc.rust-lang.org/nightly/std/vec/struct.Vec.html

// thanks @Rantanen on the Rust discord for the idea.
#[macro_export]
macro_rules! chained_iter {
    ($elem: expr) => (
        core::iter::once($elem)
    );
    ($first: expr, $($rest: expr), + $(,)?) => {
        core::iter::once($first)$(.chain(core::iter::once($rest)))*
    }
}

#[cfg(test)]
#[doc(hidden)]
mod tests {
    extern crate alloc;
    use alloc::vec;
    use alloc::vec::Vec;
    
    #[cfg(feature = "nightly")]
    use core::array::IntoIter;

    #[test]
    fn vec_equality() {
        assert_eq!(vec![1, 2, 3, 4], chained_iter![1, 2, 3, 4].collect::<Vec<_>>());
    }

    #[cfg(feature = "nightly")]
    #[test]
    fn array_equality() {
        assert_eq!(IntoIter::new([1, 2, 3, 4])
            .collect::<Vec<_>>(), chained_iter![1, 2, 3, 4].collect::<Vec<_>>());
    }
}

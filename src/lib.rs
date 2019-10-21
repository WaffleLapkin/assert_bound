//! When working with iterators/futures or other hign-generic types sometimes 
//! it's useful to assert that type of some expression implements some traits or even cast smt to `impl Trait`. 
//! 
//! This crate provides macroses for both goals — [`assert_bound`] and [`as_trait`].
//! 
//! [`assert_bound`]: ./macro.assert_bound.html
//! [`as_trait`]: ./macro.as_trait.html


/// Assert that expression implements trait(s) at compile-time.
/// 
/// ## Examples
/// ```
/// use assert_bound::assert_bound;
/// use std::fmt::Debug;
/// 
/// assert_bound!(&(); Debug + Ord + PartialEq<()>)
/// ```
/// 
/// ```
/// # use assert_bound::assert_bound;
/// trait T {}
/// impl T for () {}
/// 
/// assert_bound!(&(); T);
/// ```
/// 
/// ```compile_fail
/// # use assert_bound::assert_bound;
/// // f32/f64 doesn't implement `Eq`,
/// // so rustc will fail to compile that.
/// assert_bound!(&0.1; Eq);
/// ```
/// **NOTE**: under the hood this macro uses function that accepts `&T` where `T: *bounds*` so you **need** to pass reference and those examples will fail to compile:
/// ```compile_fail
/// # use assert_bound::assert_bound;
/// assert_bound!((); Eq);
/// ```
/// 
/// ```compile_fail
/// # use assert_bound::assert_bound;
/// trait T {}
/// impl T for () {}
/// 
/// assert_bound!(&&(); T);
/// ```
#[macro_export]
macro_rules! assert_bound {
    ($i:expr; $bound:tt $(<$param:tt $(,$p:tt)*>)? $(+ $b:tt $(<$param_:tt $(,$p_:tt)*>)?)*) => {
        // new scope, so there won't be callision with assert_bound name
        {
            fn assert_bound<__T: $bound $(<$param $($p)*>)? $(+ $b $(<$param_ $($p_)*>)?)*>(_: &__T) {}

            assert_bound($i);
        }
    }
}

/// Cast type to anonymous type (`impl Trait`) 
/// 
/// ```
/// # use assert_bound::as_trait;
/// use std::fmt::Debug;
/// 
/// println!("{:?}", as_trait!((); Debug)); // `as_trait!` return `impl Debug`
/// ```
/// 
/// ```
/// # use assert_bound::as_trait;
/// let t = as_trait!((); PartialEq<()> + PartialOrd<()>);
/// 
/// assert!(t == ());
/// assert_eq!(t.partial_cmp(&()), Some(std::cmp::Ordering::Equal));
/// ```
/// 
/// ```compile_fail
/// # use assert_bound::{as_trait, assert_bound};
/// assert_bound!(&(), Eq); // OK
/// assert_bound!(&as_trait!((); Ord), Ord); // OK
/// assert_bound!(&as_trait!((); Ord), Eq); // Error
/// ```
#[macro_export]
macro_rules! as_trait {
    ($i:expr; $bound:tt $(<$param:tt $(,$p:tt)*>)? $(+ $b:tt $(<$param_:tt $(,$p_:tt)*>)?)*) => {
        // new scope, so there won't be callision with as_trait ident
        {
            fn as_trait<__T: $bound $(<$param $($p)*>)? $(+ $b $(<$param_ $($p_)*>)?)*>(x: __T) -> impl $bound $(<$param $($p)*>)? $(+ $b $(<$param_ $($p_)*>)?)* {
                x
            }

            as_trait($i)
        }
    }
}


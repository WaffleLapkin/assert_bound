//! When working with iterators/futures or other hign-generic types sometimes it's useful to assert
//! that type of some expression implements some traits or even cast smt to `impl Trait`.
//! 
//! This crate provides macros for both goals — [`assert_bound`] and [`as_opaque`].
//!
//! [`assert_bound`]: ./macro.assert_bound.html
//! [`as_opaque`]: ./macro.as_opaque.html

/// Assert that expression implements trait(s) at compile-time.
///
/// ## Examples
/// ```
/// use assert_bound::assert_bound;
/// use std::fmt::Debug;
///
/// assert_bound!(() => Debug + Ord + PartialEq<()>);
/// ```
///
/// ```
/// # use assert_bound::assert_bound;
/// trait T {}
/// impl T for () {}
///
/// assert_bound!(() => T);
/// ```
/// **Note**: expression is **not** executed:
/// ```
/// # use assert_bound::assert_bound;
/// let mut var = 0;
/// assert_bound!({ var = 1; } => Eq);
/// assert_eq!(var, 0);
/// ```
/// However you can execute it by calling result of the macro:
/// ```
/// # use assert_bound::assert_bound;
/// let mut var = 0;
/// assert_bound!({ var = 1; } => Eq)();
/// assert_eq!(var, 1);
/// ```
/// ```compile_fail
/// # use assert_bound::assert_bound;
/// // f32/f64 doesn't implement `Eq`,
/// // so rustc will fail to compile that.
/// assert_bound!(0.1; Eq);
/// ```
/// ## Expand
/// Following code:
/// ```
/// # use assert_bound::assert_bound;
/// assert_bound!({ println!("Hewwo?") } => Ord + PartialEq<()>);
/// ```
/// expands to something like this:
/// ```
/// || {
///     fn assert_bound<T>(_: &T)
///     where
///         T: Ord,
///         T: PartialEq<()>,
///     {}
///
///     let expr = { println!("Hewwo?") };
///     assert_bound(&expr);
///     expr
/// };
/// ```
#[macro_export]
macro_rules! assert_bound {
    (
        $e:expr =>
        // One trait bound, e.g. `std::fmt::Debug`, `PartialEq<()>`
        $head:ident $( :: $tail:ident )* $( < $param:ty $(, $p:ty)* > )?
        // Zero or more trait bounds splited by `+`
        $(+ $head2:ident $( :: $tail2:ident )* $( < $param2:ty $(, $p2:ty)* > )?)*
    ) => {
        // Lambda is needed for discarding $e (lambda is returned from macro so
        // it's possible to call it in order to not discard)
        || {
            /// Assert that `T` implements traits these were given to macro
            #[inline(always)]
            fn assert_bound<__EXPR_TYPE>(_: &__EXPR_TYPE)
            where
                __EXPR_TYPE: $head $( :: $tail )* $( < $param $(, $p)* > )?,
                $( __EXPR_TYPE: $head2 $( :: $tail2 )* $( < $param2 $(, $p2)* > )? , )*
            {}

            let expr = $e;
            // Assert that $e implement traits
            assert_bound(&$e);
            // Return $e from lambda
            expr
        }
    };
}

/// Cast type to opaque type that implements trait(s) (`impl Trait + Trait2`)
///
/// ```
/// # use assert_bound::as_opaque;
///
/// println!("{:?}", as_opaque!(() => std::fmt::Debug)); // `as_opaque!` return `impl Debug`
/// ```
///
/// ```
/// # use assert_bound::as_opaque;
/// let t = as_opaque!(() => PartialEq<()> + PartialOrd<()>);
///
/// assert!(t == ());
/// assert_eq!(t.partial_cmp(&()), Some(std::cmp::Ordering::Equal));
/// ```
///
/// ```compile_fail
/// # use assert_bound::{as_opaque, assert_bound};
/// assert_bound!(&(), Eq); // OK
/// assert_bound!(&as_opaque!((); Ord), Ord); // OK
/// assert_bound!(&as_opaque!((); Ord), Eq); // Error
/// ```
///
/// ## Expand
/// Following code:
/// ```
/// # use assert_bound::as_opaque;
/// as_opaque!(() => PartialEq<()> + PartialOrd<()>);
/// ```
/// expands to something like this:
/// ```
/// fn as_opaque<T>(expr: T) -> impl PartialEq<()> + PartialOrd<()> + 'static
/// where
///     T: PartialEq<()>,
///     T: PartialOrd<()>,
///     T: 'static,
/// {
///     expr
/// }
///
/// let expr = ();
/// as_opaque(expr);
/// ```
#[macro_export]
macro_rules! as_opaque {
    (
        $e:expr =>
        // One trait bound, e.g. `std::fmt::Debug`, `PartialEq<()>`
        $head:ident $( :: $tail:ident )* $( < $param:ty $(, $p:ty)* > )?
        // Zero or more trait bounds splited by `+`
        $(+ $head2:ident $( :: $tail2:ident )* $( < $param2:ty $(, $p2:ty)* > )?)*
        // lifetime
        ; $lifetime:tt
    ) => {
            {
                /// Cast type to anonymous type that implements trait(s) these were given to macro
                #[inline(always)]
                fn as_opaque<'lifetime, __EXPR_TYPE>(expr: __EXPR_TYPE)
                    -> impl $head $( :: $tail )* $( < $param $(, $p)* > )?
                    $(+ $head2 $( :: $tail2 )* $( < $param2 $(, $p2)* > )? )*
                    + 'lifetime
                where
                    __EXPR_TYPE: $head $( :: $tail )* $( < $param $(, $p)* > )?,
                    $( __EXPR_TYPE: $head2 $( :: $tail2 )* $( < $param2 $(, $p2)* > )? , )*
                    __EXPR_TYPE: 'lifetime,
                {
                    expr
                }

                let expr = $e;
                let opaque = as_opaque::<$lifetime, _>(expr);
                opaque
            }
    };

    // Variant without lifetime (use default 'static)
    (
        $e:expr =>
        // One trait bound, e.g. `std::fmt::Debug`, `PartialEq<()>`
        $head:ident $( :: $tail:ident )* $( < $param:ty $(, $p:ty)* > )?
        // Zero or more trait bounds splited by `+`
        $(+ $head2:ident $( :: $tail2:ident )* $( < $param2:ty $(, $p2:ty)* > )?)*
    ) => {
        $crate::as_opaque!(
            $e =>
            $head $( :: $tail )* $( < $param $(, $p)* > )?
            $(+ $head2 $( :: $tail2 )* $( < $param2 $(, $p2)* > )?)*
            ; 'static
        )
    };
}

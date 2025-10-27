//! A Crate that defines the [`OnlyOne`] trait. This traits allows for result chaining in a
//! way that reduces nesting
//!
//! # Examples
//! This is a contrived example, because realistically you would just call [`std::fs::read_to_string`].
//! ```
//! # use std::io::Read;
//! use only_one::OnlyOne;
//!
//! #[derive(Debug, PartialEq, thiserror::Error)]
//! #[error(":shrug:")]
//! struct SomeError;
//!
//! impl From<std::io::Error> for SomeError {
//!     fn from(_e: std::io::Error) -> Self {
//!         SomeError
//!     }
//! }
//!
//! /// This function is equivalent to `chain`
//! fn question_mark() -> Result<usize, SomeError> {
//!     let path = "src/main.rs";
//!     let mut v = vec![];
//!     let f = std::fs::exists(path)?;
//!     if f {
//!         let mut file = std::fs::File::open(path)?;
//!         let read = file.read_to_end(&mut v)?;
//!         let bytes = v.get(0..read).ok_or(SomeError)?;
//!         let v = Vec::from(bytes);
//!         let s = String::from_utf8(v).map_err(|_| SomeError)?;
//!         Ok(s.lines().count())
//!     } else {
//!         Err(SomeError)
//!     }
//! }
//!
//! fn chain() -> Result<usize, SomeError> {
//!     let path = "src/main.rs";
//!     let mut v = vec![];
//!     if let Ok(f) = std::fs::exists(path)
//!         && f
//!     {
//!         std::fs::File::open(path)
//!             .map_err(|_| SomeError)
//!             .only(|mut file| file.read_to_end(&mut v))
//!             .only_or(|read| v.get(0..read), SomeError)
//!             .only(|bytes| {
//!                 let v = Vec::from(bytes);
//!                 String::from_utf8(v).map_err(|_| SomeError)
//!             })
//!             .map(|s| s.lines().count())
//!     } else {
//!         Err(SomeError)
//!     }
//! }
//!
//! assert_eq!(question_mark(), chain());
//! ```

#![cfg_attr(all(not(test), not(doc)), no_std)]

/// A trait that makes it easier to reduce nesting when chaining results.
///
/// This is similar using the [`std::result::Result`]'s [`and_then`](std::result::Result::and_then) and
/// [`map`](std::result::Result::map), but in a way that reduces nesting
pub trait OnlyOne<T> {
    /// The error type that will be returned. All generics for the functions in this trait must implement
    /// [`Into`] for this error type.
    type Error;
    /// Executes the closure if and only if `self` is `Ok`. If `self` is `Err`, then this function does
    /// not execute any following chains.
    ///
    /// # Examples
    /// See the [module docs](crate) for more examples.
    ///
    /// ```
    /// # use only_one::OnlyOne;
    /// # #[derive(Clone, Eq, PartialEq, Debug)]
    /// struct SomeError;
    /// # #[derive(Clone, Eq, PartialEq, Debug)]
    /// struct SomeOtherError;
    ///
    /// impl From<SomeOtherError> for SomeError {
    ///   fn from(other: SomeOtherError) -> Self { Self }
    /// }
    ///
    /// fn fallible_fn(val: usize) -> Result<usize, SomeError> {
    ///   // Some logic ...
    ///   Ok(val * 2)
    /// }
    ///
    /// fn other_fallible_fn(val: usize) -> Result<usize, SomeOtherError> {
    ///   // Some logic ...
    ///   Ok(val / 2)
    /// }
    ///
    /// let item = fallible_fn(4)
    ///              // note the following functions return `SomeOtherError`, not `SomeError`. But
    ///              // this is allowed since `SomeOtherError` can be converted into `SomeError`.
    ///              .only(other_fallible_fn)
    ///              .only(|val| if val < 100 { Ok(val / 4) } else { Err(SomeOtherError) } )
    ///              // You can still chain `only` with a type that returns `Self::Error`
    ///              .only(fallible_fn)
    ///              // But it would be the same as doing the following:
    ///              .and_then(fallible_fn);
    ///
    /// assert_eq!(Ok(4), item);
    ///
    /// ```
    fn only<U, G>(self, f: impl FnOnce(T) -> Result<U, G>) -> Result<U, Self::Error>
    where
        G: Into<Self::Error>,
        Self: Sized;

    /// Executes the closure if and only if `self` is `Some`. If `self` is `None`, returns `e`.
    ///
    /// Since this is eagerly evaluated, it should only be with trivially constructed `error`s.
    ///
    /// See the [module docs](crate) for examples.
    fn only_or<U>(
        self,
        f: impl FnOnce(T) -> Option<U>,
        error: Self::Error,
    ) -> Result<U, Self::Error>
    where
        Self: Sized;
}

impl<Good, Bad> OnlyOne<Good> for Result<Good, Bad> {
    type Error = Bad;
    #[inline]
    fn only<U, G>(self, f: impl FnOnce(Good) -> Result<U, G>) -> Result<U, Self::Error>
    where
        G: Into<Self::Error>,
        Self: Sized,
    {
        match self {
            Ok(o) => match f(o) {
                Ok(v) => Ok(v),
                Err(e) => Err(e.into()),
            },
            Err(e) => Err(e),
        }
    }

    #[inline]
    fn only_or<U>(
        self,
        f: impl FnOnce(Good) -> Option<U>,
        error: Self::Error,
    ) -> Result<U, Self::Error>
    where
        Self: Sized,
    {
        match self {
            Ok(v) => f(v).ok_or(error),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::Read;

    use crate::OnlyOne;

    #[test]
    fn idk() {
        #[derive(Debug, thiserror::Error)]
        #[error("Idk Man")]
        pub struct IdkError;

        impl From<std::io::Error> for IdkError {
            fn from(_io_err: std::io::Error) -> Self {
                IdkError
            }
        }
        impl From<std::io::ErrorKind> for IdkError {
            fn from(_io_err: std::io::ErrorKind) -> Self {
                IdkError
            }
        }

        pub fn only() -> Result<usize, IdkError> {
            let mut buf = vec![];
            let v = std::fs::File::open("b")
                .map_err(|_| IdkError)
                .only(|mut f| f.read_vectored(&mut buf))
                .only_or(|size| buf.get(0..size), IdkError)
                .map(|a| a.iter().count())
                .unwrap_or_default();
            Ok(v)
        }

        let _ = only().unwrap();
    }

    #[test]
    fn try_count_lines_in_file() {
        #[derive(Debug, PartialEq, thiserror::Error)]
        #[error(":shrug:")]
        struct TwoError;

        impl From<std::io::Error> for TwoError {
            fn from(_e: std::io::Error) -> Self {
                TwoError
            }
        }

        fn nest() -> Result<usize, TwoError> {
            let path = "src/main.rs";
            let mut v = vec![];
            let f = std::fs::exists(path)?;
            if f {
                if let Ok(mut file) = std::fs::File::open(path) {
                    if let Ok(read) = file.read_to_end(&mut v) {
                        if let Some(bytes) = v.get(0..read) {
                            let v = Vec::from(bytes);
                            if let Ok(s) = String::from_utf8(v) {
                                Ok(s.lines().count())
                            } else {
                                Err(TwoError)
                            }
                        } else {
                            Err(TwoError)
                        }
                    } else {
                        Err(TwoError)
                    }
                } else {
                    Err(TwoError)
                }
            } else {
                Err(TwoError)
            }
        }

        fn r#try() -> Result<usize, TwoError> {
            let path = "src/main.rs";
            let mut v = vec![];
            let f = std::fs::exists(path)?;
            if f {
                let mut file = std::fs::File::open(path)?;
                let read = file.read_to_end(&mut v)?;
                let bytes = v.get(0..read).ok_or(TwoError)?;
                let v = Vec::from(bytes);
                let s = String::from_utf8(v).map_err(|_| TwoError)?;
                Ok(s.lines().count())
            } else {
                Err(TwoError)
            }
        }

        fn chain() -> Result<usize, TwoError> {
            let path = "src/main.rs";
            let mut v = vec![];
            if let Ok(f) = std::fs::exists(path)
                && f
            {
                std::fs::File::open(path)
                    .map_err(|_| TwoError)
                    .only(|mut file| file.read_to_end(&mut v))
                    .only_or(|read| v.get(0..read), TwoError)
                    .only(|bytes| {
                        let v = Vec::from(bytes);
                        String::from_utf8(v).map_err(|_| TwoError)
                    })
                    .map(|s| s.lines().count())
            } else {
                Err(TwoError)
            }
        }

        assert_eq!(nest(), chain());
        assert_eq!(r#try(), chain());
    }
}

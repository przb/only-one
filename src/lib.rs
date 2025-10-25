// #![no_std]

use std::io::Read;

pub trait OnlyOne<T> {
    type Error;
    fn only<U, G>(self, f: impl FnOnce(T) -> Result<U, G>) -> Result<U, Self::Error>
    where
        G: Into<Self::Error>,
        Self: Sized;
    fn only_or<U>(self, f: impl FnOnce(T) -> Option<U>, e: Self::Error) -> Result<U, Self::Error>
    where
        Self: Sized;
}

impl<Good, Bad> OnlyOne<Good> for Result<Good, Bad> {
    type Error = Bad;
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
            Err(e) => Err(e.into()),
        }
    }

    fn only_or<U>(self, f: impl FnOnce(Good) -> Option<U>, e: Self::Error) -> Result<U, Self::Error>
    where
        Self: Sized,
    {
        match self {
            Ok(v) => f(v).ok_or(e),
            Err(e) => Err(e),
        }
    }
}

pub fn foo() -> Result<usize, IdkError> {
    let f = std::fs::File::open("b");
    if let Ok(mut f) = f {
        let mut buf = vec![];
        let f2 = f.read_vectored(&mut buf);
        if let Ok(size) = f2 {
            let array = buf.get(0..size).ok_or(IdkError);
            Ok(array.iter().count())
        } else {
            Err(IdkError)
        }
    } else {
        Err(IdkError)
    }
}

pub fn fooq() -> Result<usize, IdkError> {
    let mut f = std::fs::File::open("b")?;

    let mut buf = vec![];
    let size = f.read_vectored(&mut buf)?;

    let array = buf.get(0..size).ok_or(IdkError)?;

    Ok(array.iter().count())
}

#[derive(Debug, thiserror::Error)]
#[error("idk")]
struct FooError;
impl From<IdkError> for FooError {
    fn from(_value: IdkError) -> Self {
        FooError
    }
}
impl From<std::io::Error> for FooError {
    fn from(_value: std::io::Error) -> Self {
        FooError
    }
}

pub fn foo_o() -> Result<usize, FooError> {
    let mut buf = vec![];
    let v = std::fs::File::open("b")
        .only(|mut f| f.read_vectored(&mut buf))
        .only_or(|size| buf.get(0..size), std::io::Error::other(FooError))
        .map(|a| a.iter().count())
        .unwrap_or_default();
    Ok(v)
}

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

// #[cfg(test)]
// mod tests {
//     use super::IdkError;
//     use crate::OnlyOne;

//     #[test]
//     fn idk() {
//         fn idk_dude(_x: u32) -> Result<u32, IdkError> {
//             Ok(0)
//         }

//         fn idk_again(_x: String) -> Result<u32, IdkError> {
//             let val = 4;
//             if val > 3 {
//                 let s = std::fs::read_to_string("file");
//                 s.only(|v| idk_again(v))
//             } else {
//                 Err(IdkError)
//             }
//         }

//         // let x = 4;
//         // if x > 4 { Ok(()) } else { Err(IdkError) }
//     }
// }

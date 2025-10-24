// #![no_std]

use std::io::Read;

pub trait OnlyOne<T> {
    type Error;
    fn only<U, E, F>(self, f: F) -> Result<U, Self::Error>
    where
        E: Into<Self::Error>,
        F: FnOnce(T) -> Result<U, Self::Error>,
        Self: Sized;
    // fn finally<U>(self) -> Result<U, Self::Error>
    // where
    //     T: Into<U>;
}

impl<Good, Bad> OnlyOne<Good> for Result<Good, Bad> {
    type Error = Bad;

    fn only<U, E, F>(self, f: F) -> Result<U, Self::Error>
    where
        E: Into<Self::Error>,
        F: FnOnce(Good) -> Result<U, Self::Error>,
        Self: Sized,
    {
        match self {
            Ok(o) => f(o),
            Err(e) => Err(e.into()),
        }
    }
    // fn finally<U>(self) -> Result<U, Self::Error>
    // where
    //     Good: Into<U>,
    // {
    //     match self {
    //         Ok(v) => Ok(v.into()),
    //         Err(e) => Err(e.into()),
    //     }
    // }
}

pub fn foo() -> Result<usize, IdkError> {
    let f = std::fs::File::open("b");
    if let Ok(mut f) = f {
        let mut buf = vec![];
        let f2 = f.read_vectored(&mut buf);
        if let Ok(size) = f2 {
            let array = buf.get(0..size).unwrap();
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

    let array = buf.get(0..size).unwrap();

    Ok(array.iter().count())
}

pub fn foo_o() -> Result<usize, IdkError> {
    let mut buf = vec![];
    let x = std::fs::File::open("b")
        .only(|mut f| f.read_vectored(&mut buf))
        .only(|size| {
            let array = buf.get(0..size).unwrap();
            Ok(array.iter().count())
        })
}

#[derive(Debug, thiserror::Error)]
#[error("Idk Man")]
pub struct IdkError;

impl From<std::io::Error> for IdkError {
    fn from(_io_err: std::io::Error) -> Self {
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

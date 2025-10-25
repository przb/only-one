// #![no_std]

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

#[cfg(test)]
mod tests {
    #![allow(warnings)]
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

        fn idk_dude(_x: u32) -> Result<u32, IdkError> {
            Ok(0)
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

        pub fn foo_o() -> Result<usize, IdkError> {
            let mut buf = vec![];
            let v = std::fs::File::open("b")
                .map_err(|_| IdkError)
                .only(|mut f| f.read_vectored(&mut buf))
                .only_or(|size| buf.get(0..size), IdkError)
                .map(|a| a.iter().count())
                .unwrap_or_default();
            Ok(v)
        }

        // let x = 4;
        // if x > 4 { Ok(()) } else { Err(IdkError) }
    }
}

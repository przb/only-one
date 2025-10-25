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
            Err(e) => Err(e.into()),
        }
    }

    #[inline]
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

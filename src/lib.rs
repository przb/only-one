// #![no_std]

pub trait OnlyOne {
    type OnlyOk;
    type OnlyError;

    fn only<E, F>(self, f: F) -> Result<Self::OnlyOk, Self::OnlyError>
    where
        E: Into<Self::OnlyError>,
        F: FnOnce(Self::OnlyOk) -> Result<Self::OnlyOk, Self::OnlyError>,
        Self: Sized;
}

impl<T, E> OnlyOne for Result<T, E> {
    type OnlyOk = T;

    type OnlyError = E;

    fn only<I, F>(self, f: F) -> Result<Self::OnlyOk, Self::OnlyError>
    where
        I: Into<Self::OnlyError>,
        F: FnOnce(Self::OnlyOk) -> Result<Self::OnlyOk, Self::OnlyError>,
        Self: Sized,
    {
        match self {
            Ok(o) => f(o),
            Err(e) => Err(e.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::OnlyOne;

    #[test]
    fn idk() {
        #[derive(Debug, thiserror::Error)]
        #[error("Idk Man")]
        struct IdkError;

        impl From<std::io::Error> for IdkError {
            fn from(_io_err: std::io::Error) -> Self {
                IdkError
            }
        }

        fn idk_dude() -> Result<u32, IdkError> {
            Ok(0)
        }

        fn idk_again(_x: String) -> Result<u32, IdkError> {
            Ok(0)
        }

        let x = idk_dude();
        let x = x.and_then(|val| {
            if val > 3 {
                let s = std::fs::read_to_string("file");
                let x = s.only(|v| idk_again(v));
                // x.into()
                Ok(())
            } else {
                Err(IdkError)
            }
        });
    }
}

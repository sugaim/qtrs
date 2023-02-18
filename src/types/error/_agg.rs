pub(crate) trait AggregatableError: Sized {
    fn aggregate(values: Vec<Self>) -> Self;
    fn as_many(self) -> Result<Vec<Self>, Self>;

    fn collect<I>(it: I) -> Result<(), Self>
    where
        I: Iterator<Item = Self>,
    {
        let errs = it.collect::<Vec<_>>();
        if errs.is_empty() {
            Ok(())
        } else {
            Err(Self::aggregate(errs))
        }
    }

    fn collect_err<T, I>(it: I) -> Result<(), Self>
    where
        I: Iterator<Item = Result<T, Self>>,
    {
        let mut err = Vec::default();
        for result in it {
            match result {
                Ok(_) => {}
                Err(value) => err.push(value),
            }
        }
        if err.is_empty() {
            Ok(())
        } else {
            Err(Self::aggregate(err))
        }
    }

    fn collect_ok<T, I>(it: I) -> Result<Vec<T>, Self>
    where
        I: Iterator<Item = Result<T, Self>>,
    {
        let mut ok = Vec::default();
        let mut err = Vec::default();
        for result in it {
            match result {
                Ok(value) => ok.push(value),
                Err(value) => err.push(value),
            }
        }
        if err.is_empty() {
            Ok(ok)
        } else {
            Err(Self::aggregate(err))
        }
    }

    fn merge(fst: Self, snd: Self) -> Self {
        match (fst.as_many(), snd.as_many()) {
            (Ok(mut fst), Ok(mut snd)) => {
                fst.append(&mut snd);
                Self::aggregate(fst)
            }
            (Ok(mut fst), Err(snd)) => {
                fst.push(snd);
                Self::aggregate(fst)
            }
            (Err(fst), Ok(mut snd)) => {
                let mut result = Vec::default();
                result.reserve(snd.len() + 1);
                result.push(fst);
                result.append(&mut snd);
                Self::aggregate(result)
            }
            (Err(fst), Err(snd)) => {
                let mut result = Vec::default();
                result.reserve(2);
                result.push(fst);
                result.push(snd);
                Self::aggregate(result)
            }
        }
    }

    fn merge_result(fst: Result<(), Self>, snd: Result<(), Self>) -> Result<(), Self> {
        match (fst, snd) {
            (Ok(_), Ok(_)) => Ok(()),
            (Ok(_), Err(snd)) => Err(snd),
            (Err(fst), Ok(_)) => Err(fst),
            (Err(fst), Err(snd)) => Err(Self::merge(fst, snd)),
        }
    }
}

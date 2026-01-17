pub trait FromExt: Sized {
    fn from_<T>(t: T) -> Self
    where
        Self: From<T>,
    {
        From::from(t)
    }
}

pub trait IntoExt: Sized {
    fn into_<T>(self) -> T
    where
        Self: Into<T>,
    {
        Into::into(self)
    }
}

pub trait TryFromExt: Sized {
    fn try_from_<T>(t: T) -> Result<Self, <Self as TryFrom<T>>::Error>
    where
        Self: TryFrom<T>,
    {
        TryFrom::try_from(t)
    }
}

pub trait TryIntoExt: Sized {
    fn try_into_<T>(self) -> Result<T, <Self as TryInto<T>>::Error>
    where
        Self: TryInto<T>,
    {
        TryInto::try_into(self)
    }
}

impl<T> FromExt for T {}
impl<T> IntoExt for T {}

impl<T> TryFromExt for T {}
impl<T> TryIntoExt for T {}

pub fn from<T, U>(t: T) -> U
where
    U: From<T>,
{
    U::from(t)
}

pub fn into<T, U>(t: T) -> U
where
    T: Into<U>,
{
    T::into(t)
}

pub fn try_from<T, U>(t: T) -> Result<U, <U as TryFrom<T>>::Error>
where
    U: TryFrom<T>,
{
    U::try_from(t)
}

pub fn try_into<T, U>(t: T) -> Result<U, <T as TryInto<U>>::Error>
where
    T: TryInto<U>,
{
    T::try_into(t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_helpers() {
        let value: u8 = 7;
        let from_value: u16 = from(value);
        assert_eq!(from_value, 7);

        let into_value: u16 = into(value);
        assert_eq!(into_value, 7);

        let from_ext: u16 = u16::from_(value);
        assert_eq!(from_ext, 7);

        let into_ext: u16 = value.into_();
        assert_eq!(into_ext, 7);

        let ok: u8 = u8::try_from_(10u16).expect("valid conversion");
        assert_eq!(ok, 10);

        let conversion_result = try_from::<u16, u8>(300u16);
        assert!(conversion_result.is_err());

        let ok: u8 = 8u16.try_into_().expect("valid conversion");
        assert_eq!(ok, 8);

        let conversion_result = 300u16.try_into_::<u8>();
        assert!(conversion_result.is_err());
    }
}

/// Analogous to regular `map` but works on any type.  
/// Transforms one type into another.
///
/// # Examples
///
/// ```
/// use std::ops::Add;
/// use std::time::Duration;
/// use maptypings::MapType;
///
/// let time = 10u64;
/// let time: Duration = time.add(15).map_type(|t| Duration::from_millis(t));
///
/// // Or
/// let num: Option<u32> = 55.map_type(Some);
/// ```
pub trait MapType<M> {
    /// Converts one type into another.
    fn map_type<N>(self, f: impl FnOnce(M) -> N) -> N;
}

impl<M> MapType<M> for M {
    fn map_type<N>(self, f: impl FnOnce(Self) -> N) -> N {
        f(self)
    }
}

/// Wraps type in `Option` and returns `None` if condition is true.
///
/// # Examples
///
/// ```
/// use maptypings::NoneIf;
///
/// let s: String = "".to_owned();
/// let optional: Option<String> = s.none_if(String::is_empty);
/// ```
pub trait NoneIf<T> {
    /// Returns `None` on `cond == true`.
    fn none_if(self, cond: impl Fn(&T) -> bool) -> Option<T>;
}

impl<T> NoneIf<T> for T {
    fn none_if(self, cond: impl Fn(&Self) -> bool) -> Option<Self> {
        match cond(&self) {
            true => None,
            _ => Some(self),
        }
    }
}

/// Wraps type in `Result` and returns `Err` if condition is true.
///
/// # Examples
///
/// ```
/// use maptypings::ErrIf;
///
/// let name: &str = "John";
/// let err: String = "Error: something went wrong".to_owned();
///
/// let name: Result<&str, String> = name.err_if(|s| s.is_empty(), err);
/// ```
pub trait ErrIf<T> {
    /// Returns a given error on `cond == true`.
    fn err_if<E>(self, cond: impl Fn(&Self) -> bool, err: E) -> Result<T, E>;
}

impl<T> ErrIf<T> for T {
    fn err_if<E>(self, cond: impl Fn(&Self) -> bool, err: E) -> Result<Self, E> {
        match cond(&self) {
            true => Err(err),
            _ => Ok(self),
        }
    }
}

/// Maps any value to `()`.
pub trait ForgetValue {
    /// Forgets any value.
    fn forget_val(self);
}

impl<T> ForgetValue for T {
    fn forget_val(self) {}
}

/// Wraps type in `Result`.  
/// It exists to complement convenience of mapping any type into `Option`.
///
/// # Examples
///
/// ```
/// use maptypings::WrapInRes;
///
/// fn ret_str() -> Result<String, String> {
///     "hello"
///         .to_owned()
///         .in_ok()
/// }
/// ```
pub trait WrapInRes<T> {
    /// Wraps type in `Result::Ok`.
    fn in_ok<E>(self) -> Result<T, E>;
    /// Wraps type in `Result::Err`.
    fn in_err<O>(self) -> Result<O, T>;
}

impl<T> WrapInRes<T> for T {
    fn in_ok<E>(self) -> Result<Self, E> {
        Ok(self)
    }
    fn in_err<O>(self) -> Result<O, Self> {
        Err(self)
    }
}

/// Swaps `Ok` and `Err`.  
/// May be useful in rare cases where the difference between `Result::Ok` and
/// `Result::Err` is shallow, or when you have things like `Option::Some(Error)`.
///
/// # Examples
///
/// ```
/// use maptypings::SwapRes;
///
/// let err = "Error: something went wrong".to_owned();
/// let opt_err: Option<String> = Some(err);
///
/// let res: Result<u32, String> = opt_err.ok_or(16).swap_res();
/// ```
pub trait SwapRes<T, E> {
    /// Swaps `Ok` and `Err` variants.
    fn swap_res(self) -> Result<T, E>;
}

impl<T, E> SwapRes<E, T> for Result<T, E> {
    fn swap_res(self) -> Result<E, T> {
        match self {
            Ok(t) => Err(t),
            Err(e) => Ok(e),
        }
    }
}

/// Turns `Option<T>` into `Result`, with respect to whether `T` should be
/// an `Ok` or an `Err`.
///
/// # Examples
///
/// ```
/// use maptypings::AddToRes;
///
/// let opt: Option<u32> = Some(16);
///
/// let res: Result<u32, String> = opt.add_err("Error: ...".to_owned());
/// // Or
/// let res: Result<String, u32> = opt.add_ok("foo".to_owned());
/// ```
pub trait AddToRes<T> {
    /// Turns `Option<T>` into `Result<O, T>`.
    fn add_ok<O>(self, ok: O) -> Result<O, T>;
    /// Turns `Option<T>` into `Result<T, E>`.
    fn add_err<E>(self, err: E) -> Result<T, E>;
}

impl<T> AddToRes<T> for Option<T> {
    fn add_ok<O>(self, ok: O) -> Result<O, T> {
        match self {
            None => Ok(ok),
            Some(e) => Err(e),
        }
    }
    fn add_err<E>(self, err: E) -> Result<T, E> {
        self.ok_or(err)
    }
}

/// Mutates value and returns it back.
///
/// # Examples
///
/// ```
/// use maptypings::Mutate;
///
/// let sorted = vec![4,3,5,7,6,0,7,6,6,3,5,4,3,2,6,5,4,3].mutate(|v| v.sort());
/// ```
pub trait Mutate<T> {
    /// Mutates value and returns it back.
    fn mutate<R>(self, f: impl FnOnce(&mut T) -> R) -> T;
}

impl<T> Mutate<T> for T {
    fn mutate<R>(self, f: impl FnOnce(&mut Self) -> R) -> Self {
        let mut val = self;
        f(&mut val);
        val
    }
}

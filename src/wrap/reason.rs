use std::fmt::{Display, Debug};
pub use self::Reason::*;

/// success or reason-known failure
/// T => reason type
/// 
/// # Example
/// ```rust
/// fn action()-> Reason<&'static str> 
/// { 
///     return Failure("some reason");
/// }
/// 
/// fn main()
/// {
///     let result = action();
///     result.on_success(|| println!("success"));
///     result.on_failure(|reason| println!("failure: {reason:?}"));
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Reason<T>
{
    Success, 
    Failure(T)
}

impl<T> Reason<T> 
{
    /// run action if success
    pub fn on_success( &self, action: impl Fn() )-> &Self
    {
        if let Self::Success = self
        {
            action();
        }
        return self;
    }

    /// run action if failure
    pub fn on_failure( &self, action: impl Fn(&T) )-> &Self
    {
        if let Self::Failure(reason) = self
        {
            action(reason);
        }
        return self;
    }
}

impl<T> Display for Reason<T> where T: Debug
{
    fn fmt( &self, f: &mut std::fmt::Formatter<'_> ) -> std::fmt::Result 
    {
        match self 
        {
            Self::Success => write!(f, "Success"),
            Self::Failure(reason) => write!(f, "Failure[{reason:?}]")
        }
    }
}
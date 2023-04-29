//!There are all utils of this crate

use std::hash::Hash;

use crate::{Fput, Input};

/// Convert  type that implement `Into<Input<...>>` to `Input`
/// # Examples
///```
/// use maller::{input, Maller};
///
/// let input1=(12,input(|x|x*2));
/// let input2=(14,input(|x|x*2));
///
/// let mut maller=Maller::from_iter([input1,input2]);
///
/// assert_eq!(maller.call(&12).unwrap(),24);
/// ```
pub fn input<'a, 'b, T, R, P>(param: P) -> Input<'a, 'b, T, R>
    where P: Into<Input<'a, 'b, T, R>>,
          T: Eq + Hash

{
    param.into()
}

/// See [`Input::new`](Input::new) for more details
pub fn new_input<'a, 'b, T, R>(param: Fput<'a, 'b, T, R>) -> Input<'a, 'b, T, R>
    where T: Eq + Hash {
    Input::new(param)
}
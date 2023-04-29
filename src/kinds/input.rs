//!there is the Input struct
//!<br>and other related types


use std::hash::Hash;

pub(crate) type Fput<'a,'b,T,R>= Box<dyn FnMut(&'b T)->R+'a>;

///type that includes a key and function
pub struct Input<'a,'b,T,R>
    where T:Eq+Hash
{
    inner:(T,Fput<'a,'b,T,R>)
}

impl<'a,'b,T,R>  Input<'a,'b,T,R>
    where T:Eq+Hash
{
    ///allows to destroy the input and get inner values
   pub(crate) fn destroy(self)->(T,Fput<'a,'b,T,R>){
        self.inner
    }

    ///allows to create new Input from  `(T,Box<dyn FnMut(...)>)`
    ///
    /// ### you shouldn't use it because`
    ///1. __if you have `(T,F:FnMut(...))` or `(T,Box<F:FnMut(...)>)`
    /// you can use [`input()`][1] from `utils`__
    ///2. __you can use shorter  [`new_input()`][2] from `utils`__<br>
    ///
    ///3. It's always better to use [`input()`][1] because it
    /// takes any type that can convert into Input<...><br><br>
    ///
    /// __But if you have `(T,Box<F:FnMut(...)>)`
    /// it converts into `(T,Box<Box<F:FnMut(...)>>)`__<br>
    /// Whatever it happens, use [`new_input()`][2]`
    ///```
    /// use maller::{Maller, new_input};
    ///
    /// let mut mr=Maller::new();
    ///
    /// let input=Box::new(|x|x+100);
    /// let input=new_input((0,input));
    ///
    /// mr.insert(input);
    ///
    /// ```
    ///
    /// [1]: crate::utils::input
    /// [2]: crate::new_input
    ///
    ///
    ///
    /// # Examples
    ///```
    /// use maller::{Input, input, Maller};
    ///
    /// let mut maller=Maller::new();
    ///
    /// let input=input((12,|x|x+2));
    /// maller.insert(input);
    /// ```
   pub fn new(param:(T,Fput<'a,'b,T,R>))->Self{
       Self{
           inner:param
       }
   }
}


impl<'a,'b,T,R,F> From<(T, F)> for Input<'a,'b,T,R>
    where T:Eq+Hash,
          F:FnMut(&'b T)->R+'a
{
    fn from(value: (T, F)) -> Self {
        Self::new ((value.0, Box::new(value.1)))
    }
}

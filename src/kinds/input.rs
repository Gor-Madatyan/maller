//!There is the `Input` struct
//!<br>and other related types


use std::hash::Hash;

pub(crate) type Fput<'a,'b,T,R>= Box<dyn FnMut(&'b T)->R+'a>;

///Type that have a inner closure<br>
/// ___
/// if Input<...> impl From<**TYPE**> then **TYPE** can be used as input closure
pub struct Input<'a,'b,T,R>
    where T:Eq+Hash
{
    inner:Fput<'a,'b,T,R>
}

impl<'a,'b,T,R>  Input<'a,'b,T,R>
    where T:Eq+Hash,

{
    ///Allows to create new Input from  `Box<dyn FnMut(&T)->R>`
    ///
    /// ### you mustn't use it because`
    ///1. __if you have `F:FnMut(...)`
    /// you can use [`input()`][1] from `utils`__
    ///
    ///2. __you can use shorter  [`new_input()`][2] from `utils`__<br>
    ///
    ///3. __It is almost always better to use [`input()`][1] because it
    /// takes any type that can convert into Input<...>__<br><br>
    ///
    /// __But if you have `Box<F:FnMut(...)>` and push it in [`input()`][1]
    /// it converts into `Box<Box<dyn FnMut(...)>>`, because `Box<F:Fn...>` impl `Fn...` <br>
    /// Whatever it happens, use [`new_input()`][2]`__
    ///
    ///[1]:crate::utils::input
    ///[2]:crate::new_input
    ///
    ///___
    /// # Examples
    ///
    ///```
    /// use maller::{Maller, new_input};
    ///
    /// let mut mr=Maller::new();
    ///
    /// let input=Box::new(|x|x+100);
    /// let input=new_input(input);
    ///
    /// mr.insert(0,input);
    ///
    /// ```
   pub fn new(param:Fput<'a,'b,T,R>)->Self{
       Self{
           inner:param
       }
   }


    ///Allows to run closure in input with parameter
    /// # Example
    ///```
    /// use maller::input;
    /// let mut inp=input(|x|x+2);
    ///
    /// assert_eq!(inp.run(&23),25);
    /// ```
    pub fn run(& mut self,key:&'b T)->R{
    let f=&mut self.inner;
    f(key)
}
}


impl<'a,'b,T,R,F> From<F> for Input<'a,'b,T,R>
    where T:Eq+Hash,
          F:FnMut(&'b T)->R+'a
{
    fn from(value:F) -> Self {
        Self::new(Box::new(value))
    }
}

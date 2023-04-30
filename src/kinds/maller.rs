//! Here you can find all types <br>
//! related to the main implement

use std::collections::HashMap;
use std::hash::Hash;
use std::ops::{Deref, DerefMut};

use crate::kinds::Input;

///The main structure to have use
pub struct Maller<'a,'b,T,R>
    where
        T:Eq+Hash{

    inner: HashMap<T, Input<'a,'b,T,R>>
}

impl<'a,'b,T,R> Maller<'a,'b,T,R>
    where
        T:Eq+Hash, {

    ///Allows to create new default Maller,
    /// you must enter the type, or add a value
    /// # Examples
    /// ```
    /// use maller::Maller;
    /// let mut mr=Maller::<&str,()>::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }



    ///Allows to call a suitable closure by key,
    /// the ref on key have to pass to closure.
    /// return `Some(R)` if closure is found
    ///else `None`
    ///
    /// # Examples
    /// use maller::{input, Maller};
    /// let mut con=0;
    ///
    /// let c1=|x|{con+=1;2};
    /// let c2=|x|3;
    ///
    /// let mut maller=Maller::from_iter(
    ///    [(898, input(c1)),
    ///     (500, input(c2))]
    /// );
    ///
    ///assert_eq!(maller.call(&898).unwrap(),2);
    ///
    /// ```
    pub fn call(&mut self, param: &'b T) -> Option<R> {
        self.inner.get_mut(param).map(|x| x.run(param))
    }

    ///Allows to get inner HashMap
    pub fn inner(self)->HashMap<T, Input<'a,'b,T,R>>{
        self.inner
    }

}

impl<'a,'b,T,R> Default for Maller<'a,'b,T,R> where
    T:Eq+Hash, {
    fn default() -> Self {
        Self{
            inner:HashMap::default()
        }
    }
}

impl<'a,'b,T,R> Deref for Maller<'a,'b,T,R> where
    T:Eq+Hash, {
    type Target = HashMap<T,Input<'a,'b,T,R>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
    }


impl<'a,'b,T,R> DerefMut for Maller<'a,'b,T,R> where
    T:Eq+Hash, {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}



impl<'a,'b,T,R> FromIterator<(T,Input<'a,'b,T,R>)> for Maller<'a,'b,T,R>
    where
        T:Eq+Hash
        {
            fn from_iter<I: IntoIterator<Item=(T, Input<'a, 'b, T, R>)>>(iter: I) -> Self {
                Self{
                    inner:HashMap::from_iter(iter)
                }
            }
        }

impl<'a,'b,T,R> From<HashMap<T,Input<'a,'b,T,R>>> for Maller<'a,'b,T,R>
    where
    T:Eq+Hash
{
    fn from(inner: HashMap<T, Input<'a, 'b, T, R>>) -> Self {
        Self{
            inner
        }
    }
}
//! here you can find all types <br>
//! related to the main implement

use std::collections::HashMap;
use std::hash::Hash;
use std::ops::{Deref, DerefMut};

use crate::kinds::{Fput,Input};

///the main structure to have use
pub struct Maller<'a,'b,T,R>
    where
        T:Eq+Hash{

    inner: HashMap<T, Fput<'a,'b,T,R>>
}

impl<'a,'b,T,R> Maller<'a,'b,T,R>
    where
        T:Eq+Hash, {

    ///allows to create new default Maller,
    /// ___
    /// you must enter the type, or add a value
    /// # Examples
    /// ```
    /// use maller::Maller;
    /// let mut mr=Maller::<&str,()>::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    ///allows to
    ///   __add closure__ and
    ///   __change an existing closure__
    ///___
    /// if closure already exists, function will return
    /// `Some(F)`, else `None`
    ///
    /// # Examples
    ///```
    /// use maller::Maller;
    /// let mut mr=Maller::new();
    ///
    ///mr.insert(("foo",|x|println!("hello {x}")));
    ///
    ///assert!(mr.insert(("foo",|x|println!("2 hl"))).is_some());
    /// ```
    pub fn insert<P>(&mut self, val:P) -> Option<Fput<'a,'b,T,R>>
            where P:Into<Input<'a,'b,T,R>>
    {
        let (key,val)=val.into().destroy();
        self.inner.insert(key, val)
    }

    ///allows to call a suitable closure by key,
    /// the ref on key have to pass to closure.
    /// ___
    /// return `Some(R)` if closure is found
    ///else `None`
    ///
    /// # Examples
    ///```
    /// use maller::{input, Maller};
    /// let mut con=0;
    ///
    /// let c1=|x|{con+=1;2};
    /// let c2=|x|3;
    ///
    /// let mut maller=Maller::from_iter(
    ///    [input((898, c1)),
    ///     input((500, c2))]
    /// );
    ///
    ///assert_eq!(maller.call(&898).unwrap(),2);
    ///
    /// ```
    pub fn call(&mut self, param: &'b T) -> Option<R> {
        self.inner.get_mut(param).map(|x| x(param))
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
    type Target = HashMap<T,Fput<'a,'b,T,R>>;

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



impl<'a,'b,T,R> FromIterator<Input<'a,'b,T,R>> for Maller<'a,'b,T,R>
    where
        T:Eq+Hash
        {
    fn from_iter<I: IntoIterator<Item=Input<'a,'b,T,R>>>(iter: I) -> Self {
        let iter=iter.into_iter().map(|x|x.destroy());
        Self{
         inner:HashMap::from_iter(iter)
        }
    }
        }
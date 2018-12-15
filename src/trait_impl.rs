use std::ops::{Index, Deref};
use std::process;
use std::fmt;
use super::*;  
use dim_traits::Coord;

// impl<'a,T> Deref for OwnedMatrix<T>{

//     type Target = MatrixView<'a,T>;

//     fn deref(&self) -> &MatrixView<'a,T>{
//         unimplemented!()
//     }

// }

//implements the Index method for an N dimension matrix.
impl<'a,A:Data> Index<&'a [usize]> for BaseMatrix<A,Vec<usize>>{
    type Output = <A as Data>::Elem;    
    fn index(& self, index:&[usize]) -> &<A as Data>::Elem{
        let data = self.raw_data();        
        &data[index.index_checked( self.get_shape())]
    }
}

impl<'a,T:fmt::Display> Index<usize> for OwnedCollection<T>{
    type Output = <OwnedCollection<T> as Data>::Elem;
    fn index(&self, index: usize) -> &<OwnedCollection<T> as Data>::Elem{
        &self.0[index]
    }
}

impl<T:fmt::Display> IntoIterator for Matrix<T>{
    type Item = T;
    type IntoIter = ::std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_raw_data().into_iter()
    }
}

impl<T:fmt::Display> IntoIterator for OwnedCollection<T>{
    type Item = T;
    type IntoIter = ::std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a,A:Data> fmt::Display for BaseMatrix<A,Vec<usize>> 
    {
    fn fmt(&self, f: &mut fmt::Formatter ) ->fmt::Result {
        println!("Shape = {:?}",self.get_shape());
        for n in 0..self.get_dim()[0] {
            for m in 0..self.get_dim()[1]{

                
                write!(f,"{} ", self[&[n,m]])?;
            }
            write!(f,"\n")?;
       }
       write!(f,"")
    }
}


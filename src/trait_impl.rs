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
impl<'a,T, A:MatCollection<T>> Index<&'a [usize]> for Matrix<T,A,Vec<usize>>{
    type Output = T;    
    fn index(& self, index:&[usize]) -> &T{
        let data = self.raw_data();        
        &data[index.index_checked( self.get_shape())]
    }
}

impl<T> IntoIterator for OwnedMatrix<T>{
    type Item = T;
    type IntoIter = ::std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_raw_data().into_iter()
    }
}

impl<'a,A,T:MatCollection<A>> fmt::Display for Matrix<A,T,Vec<usize>> 
    where
        A:fmt::Display,
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

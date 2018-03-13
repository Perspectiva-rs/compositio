use std::ops::Index;
use std::process;
use std::fmt;
use Matrix;  
use MatCollection;
use dim_traits::Coord;



//implements the Index method for an N dimension matrix.
impl<'a,T, A:MatCollection<T>> Index<&'a [usize]> for Matrix<T,A,Vec<usize>>{
    type Output = T;    
    fn index(& self, index:&[usize]) -> &T{
        let data = self.raw_data();
        
        &data[index.index_checked( self.dim() ).unwrap()]
    }
}

impl<'a,A,T:MatCollection<A>> fmt::Display for Matrix<A,T,Vec<usize>> 
    where
        A:fmt::Display,
    {
    fn fmt(&self, f: &mut fmt::Formatter ) ->fmt::Result {
        
        for n in 0..self.dim()[0] {
            for m in 0..self.dim()[1]{
                
                write!(f,"{} ", self[&[n,m]])?;
            }
            write!(f,"\n")?;
       }
       write!(f,"")
    }
}

use std::ops::Index;
use std::process;
use std::fmt;
use Matrix;  
use dim_traits::Coord;



//implements the Index method for an N dimension matrix.
impl<'a,'b, T> Index<&'a [usize]> for Matrix<T,Vec<usize>>{
    type Output = T;    
    fn index(& self, index:&[usize]) -> &T{
        let data = self.raw_data();
        &data[index.index_checked( self.dim() ).unwrap_or_else(|err| 
            {
                eprintln!("{}", err);
                process::exit(1)})]
            }   
}

impl<'a,T> fmt::Display for Matrix<T,Vec<usize>> 
    where
        T:fmt::Display,
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

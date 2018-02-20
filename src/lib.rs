use std::fmt;
use std::ops::Index;
use std::borrow::Borrow;
use std::borrow::BorrowMut;

pub trait Coord{
    type Dimension: Dim;
    fn index(&self, dim: &Self::Dimension) -> Option<usize>;

}

impl Coord for [usize;2]{
    type Dimension = [usize;2];
    fn index(&self, dim: &[usize;2]) -> Option<usize>{
        let array = dim.as_array();
        if dim.check_bounds(&self) {
            Some(self[0] + self[1] * array[1])  
        }else{
            None
        }
    }
}

pub trait Dim{
    type Coord: Coord;
    fn as_array(&self) -> &[usize];
    fn as_mut_array(&mut self) -> &mut [usize];
    fn increase_size(&mut self, axis: usize, inc: usize);
    fn get(&self,axis:usize) -> usize;
    fn check_bounds(&self, coord: &Self::Coord)->bool;
}

impl Dim for [usize;2] {
    type Coord = [usize;2];
    
    fn as_array(&self) -> &[usize]{
        self.borrow()
    }
    fn as_mut_array(&mut self) -> &mut [usize] {
        self.borrow_mut()
    }

    fn increase_size(&mut self, axis: usize, inc: usize){
        self[axis] = self[axis] + inc;
    }
    fn get(&self,axis:usize) -> usize{
        self[axis]
    }
    
    fn check_bounds(&self, coord: &Self::Coord)->bool{
        if self.get(0) >  coord[0] && self.get(1) >  coord[1]{
            true
        }else{
            false
        }
    }
}

#[derive(Debug)]
pub struct Matrix<T,D:Dim>{
    pub data: Vec<T>,
    pub dim: D
}
impl<T:Sized,D:Dim> Matrix<T,D>{
    pub fn new(data:Vec<T>,dim:D) -> Matrix<T,D> {
        Matrix{data , dim}
    }
    pub fn dim(&self) -> &D {
        &self.dim
    }
    fn mut_dim(&mut self) -> &mut D {
        &mut self.dim
    }

    pub fn data(&self) -> &Vec<T>{
        &self.data
    }
    pub fn data_mut(&mut self) -> &mut Vec<T> {
        &mut self.data
    }

    pub fn append_column(&mut self, column: &mut Vec<T>) {
        
        {
            let size = self.mut_dim().as_mut_array();
            if size == [0usize,0usize]{
                size[1] = column.len();
            }else{
                if column.len() != size[1] {
                    panic!("Matrix columns have variable length");
                }
            }
        }

        self.dim.increase_size(0,1);
        self.data.append(column);
        
    }
}

impl<T,C,D> Index<C> for Matrix<T,D>
    where 
        T: Sized,
        C: Coord<Dimension = D>,
        D: Dim
    {

    type Output = T;    
    fn index(&self, index: C) -> &T{
        let data = self.data();
        &data[index.index( self.dim() ).unwrap()]
    }
}

impl<T:fmt::Display> fmt::Display for Matrix<T,[usize;2]> {
    fn fmt(&self, f: &mut fmt::Formatter ) ->fmt::Result {
        
        for n in 0..self.dim.get(1) {
            for m in 0..self.dim.get(0){
                
                write!(f,"{} ", self[[n,m]])?;
            }
            write!(f,"\n")?;
       }
       write!(f,"")
    }
}
// [1, 1, 1, 1, 1] => Vec<>
// [1, 1, 1, 1, 1]
// [1,1,1,1;1,1,1,1;1,1,1,1] 1,1,1,1,1; mat!(mat1|mat2)
#[allow(unused_macros)]
macro_rules! mat {
        ($($($x:expr),*);*) => {
            {
                let mut matrix = Matrix::new(vec![],[0usize, 0usize]);
                $(let mut vector = vec![$($x),*];
                matrix.append_column(&mut vector);
                );*
                println!("{}",matrix );
                matrix
            }
        };
        
    }

#[macro_use]
#[cfg(test)]


mod tests {
    
    use super::*;
    
    //use super::macros;
    #[test]
    fn vector() {
        let matrix = Matrix::new(vec![2,3,4],[1 as usize, 3 as usize]);
        let macro_matrix = mat![2,3,4];
        println!("{}",macro_matrix );

        assert_eq!(matrix.data, macro_matrix.data);
        //assert_eq!(matrix.dim(), macro_matrix.dim());

    }
    #[test]
    fn matrix() {
        let matrix = Matrix::new(vec![2,3,4,5,6,7,8,9,10],[3 as usize, 3 as usize]);
        let macro_matrix = mat![2,3,4;5,6,7;8,9,10];
        
        assert_eq!(matrix.dim(), macro_matrix.dim());
       // assert_eq!(matrix.data, macro_matrix.data);

    }
    #[test]
    fn indexing() {
        let matrix = mat![2,3,4;5,6,7;8,9,10];
        println!("{}",matrix );
        
        assert_eq!(5, matrix[[1,0]]);
    }
}

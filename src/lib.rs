use std::fmt;
use std::ops::Index;
use std::borrow::Borrow;

pub trait Coord<D>{
    fn index(&self, dim: &D) -> Option<usize>;

}

impl<D:Dim> Coord<D> for [usize;2]{
    fn index(&self, dim: &D) -> Option<usize>{
        let array = dim.as_array();
        self[0] + self[1] * array[1]
    }
}

pub trait Dim{
    fn as_array(&self) -> &[usize];
    fn increase_size(&mut self, axis: usize, inc: usize);
    fn get(&self,axis:usize) -> usize;
}

impl Dim for [usize;2] {
    fn as_array(&self) -> &[usize]{
        self.borrow()
    }
    fn increase_size(&mut self, axis: usize, inc: usize){
        self[axis] = self[axis] + inc;
    }
    fn get(&self,axis:usize) -> usize{
        self[axis]
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
    pub fn data(&self) -> &Vec<T>{
        &self.data
    }
    pub fn data_mut(&self) -> &mut Vec<T> {
        &mut self.data
    }

    pub fn append_column(&mut self, column: &mut Vec<T>) {
        let size = self.dim.as_array();
        if size == [0usize,0usize]{
            size[1] = column.len();
        }else{
            if column.len() != size[1] {
                panic!("Matrix columns have variable length");
            }
        }

        self.data.append(column);
        self.dim.increase_size(0,1);
    }
}

impl<T,C,D> Index<C> for Matrix<T,D>
    where 
        T: Sized,
        D : Dim,
        C : Coord<D>,
    {

    type Output = T;    
    fn index(&self, index: C) -> &T{
        let data = self.data();
        &data[index.index( self.dim() ).unwrap()]
    }
}

impl<T:fmt::Display,D:Dim> fmt::Display for Matrix<T,D> {
    fn fmt(&self, f: &mut fmt::Formatter ) ->fmt::Result {
        
        for n in 0..self.dim.get(1) {
            for n in 0..self.dim.get(0){
                write!(f,"{} ", self.data[n])?;
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
    fn create_vector() {
        let matrix = Matrix::new(vec![1,3,4],[1 as usize, 3 as usize]);
        let macro_matrix = mat![2,3,4];
         println!("{}",macro_matrix );

        assert_eq!(matrix.data, macro_matrix.data);
        assert_eq!(matrix.size(), macro_matrix.size());

    }
    #[test]
    fn matrix() {
        let matrix = Matrix::new(vec![2,3,4,5,6,7,8,9,10],[3 as usize, 3 as usize]);
        let macro_matrix = mat![2,3,4;5,6,7;8,9,10];
        
        assert_eq!(matrix.size(), macro_matrix.size());
        assert_eq!(matrix.data, macro_matrix.data);

    }

    fn create_matrix() {
        let matrix = mat![2,3,4;5,6,7;8,9,10];
        
        assert_eq!(5, matrix[[1,2]]);
    }
}

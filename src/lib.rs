



#[macro_use] mod macros;
mod trait_impl;
mod dim_traits;

use dim_traits::Dim;
use std::ops::Index;
use std::rc::Rc;
use std::marker::PhantomData;


///Private trait to encapsulate collections that can hold matrix data

pub trait MatCollection<T>: Index<usize, Output = T>{
}

impl<T> MatCollection<T> for Vec<T>{
}

impl<T> MatCollection<T> for BorrowedData<T>{}

#[derive(Debug,PartialEq)]
pub struct BorrowedData<T>(Box<Vec<T>>);

impl<T> Index<usize> for BorrowedData<T> {
    type Output = T;
    
    fn index(&self, index: usize) -> &T {
        &self.0[index]
    }
}

type OwnedMatrix<T> = Matrix<T,Vec<T>,Vec<usize>>;
type MatrixView<T> =  Matrix<T, BorrowedData<T>,Vec<usize>>;

/// An n dimensional array.
/// The array is a general container of data.
/// The array can grow.
/// The array can be indexed by using a number of coordinates equal to the dimension of the matrix.
#[derive(Debug,PartialEq)]
pub struct Matrix<T,A:MatCollection<T>, D>{
    data: A,
    dim: D,
    dummy: PhantomData<T>,
}
impl<T, A:MatCollection<T>> Matrix<T,A,Vec<usize>>{

    /// constructor method for owned arrays.
    pub fn new(data:A,dim:Vec<usize>) -> Matrix<T,A,Vec<usize>> {
        Matrix{data:data , dim:dim,dummy:PhantomData}
    }

    /// returns the dimensions of the matrix as a slice.
    pub fn dim(& self) -> &[usize] {
        &self.dim 
    }
    /// returns thre dimensions of the matrix as a mutable slice.
    fn mut_dim(& mut self) -> & mut [usize] {
        &mut self.dim
    }

    /// returns a reference to the matrices raw_data. 
    pub fn raw_data(&self) -> &A{
        &self.data 
    }
    /// returns a mutable reference to the matrices raw_data.
    pub fn raw_data_mut(&mut self) -> &mut A{
        &mut self.data
    }
    pub fn get_column(&self) -> MatrixView<T>{
        unimplemented!()
    }
    
}

impl<T> OwnedMatrix<T>{
    ///appends a column to a matrice from raw_data.
    pub(crate) fn append_column_from_raw(&mut self, column: &mut Vec<T>) {
        {let  dim = self.mut_dim();
            if !dim.is_empty() {
                dim[0] = column.len();
            }else{
                if column.len() != dim[0]{
                        panic!("Matrix columns have variable length");
                }
            }
        dim.increase_size(1,1); 
        }
        self.data.append(column);
        
    }
    pub fn append_line(&mut self, line: &mut Vec<T>){
        unimplemented!();
    }
}
type Matrixf32 = Matrix<f32,Vec<f32>,Vec<usize>>;
impl Matrixf32{
    /// creates matrix of with dimensions dim filled with zeros.
    pub fn zeros(dim:Vec<usize>) -> Matrixf32{
        let length = dim.iter().product();

        Matrix::new(vec![0.0;length],dim)
    }
    ///creates matrix with dimensions dim filled with ones.
     pub fn ones(dim:Vec<usize>)-> Matrixf32{
        let length = dim.iter().product();

        Matrix::new(vec![1.0;length], dim)
    }
}
type Matrixi32 = Matrix<i32, Vec<i32>,Vec<usize>>;
impl Matrixi32{
    /// creates matrix of with dimensions dim filled with zeros.
     pub fn zeros(dim:Vec<usize>)-> Matrixi32{
        let length = dim.iter().product();

        Matrix::new(vec![0;length], dim)
    }
    ///creates matrix with dimensions dim filled with ones.
     pub fn ones(dim:Vec<usize>)-> Matrixi32{
        let length = dim.iter().product();
        Matrix::new(vec![1;length], dim)
    }
}






#[cfg(test)]
mod tests {
    
    use super::*;
    
    
    //use super::macros;
    #[test]
    fn indexing() {
        let matrix = mat![2,3,4;5,6,7;8,9,10];
        println!("{}",matrix);
        
        assert_eq!(5, matrix[&[0,1]]);
    }
    #[test]
    fn indexing_3d() {
        let matrix = Matrix::new(vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18],vec![3,3,2]);
        assert_eq!(3,matrix[&[2,0,0]]);
        assert_eq!(18,matrix[&[2,2,1]]);
    }
    #[test]
    fn indexing_5d() {
        let matrix = Matrix::new(vec![1,2,3,4, 5,6,7,8,  9,10,11,12, 13,14,15,16],vec![2,2,2,2]);
        
        assert_eq!(16,matrix[&[1,1,1,1]]);
    }
    #[test]
    fn zeros_and_ones_matrix(){
        let ones = Matrixf32::ones(vec![10,10]);
        let zeros = Matrixf32::zeros(vec![10,10]);
        println!("\n{}\n{}",ones, zeros);
    }

    // #[test]
    // fn vector_slice_test(){
    //     let matrix = mat![1,2,3;4,5,6];
    //     let slice = matrix.get_column(0);
    //     let matrix_test = mat![1,2,3];
    //     assert_eq!(slice, matrix_test);
        
    // }

}

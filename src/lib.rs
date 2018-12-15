



#[macro_use] mod macros;
mod trait_impl;
mod dim_traits;
mod array_views;
mod shape;


use dim_traits::Dim;
use std::ops::Index;
use std::rc::Rc;
use array_views::{SliceParameters,MatrixView};
use std::marker::PhantomData;

use shape::Shape;


///Private trait to encapsulate collections that can hold matrix data

pub trait MatCollection<T>: Index<usize, Output = T>{
}
impl<T> MatCollection<T> for Vec<T>{
}
pub trait Data{
    type Elem;
}
pub struct OwnedCollection<T>(Vec<T>);

pub type OwnedMatrix<T> = Matrix<T,Vec<T>,Vec<usize>>;

impl<T> Data for OwnedCollection<T>{
    type Elem = T;
}




/// An n dimensional array.
/// The array is a general container of data.
/// The array can grow.
/// The array can be indexed by using a number of coordinates equal to the dimension of the matrix.
#[derive(Debug,PartialEq)]
pub struct Matrix<T,A:MatCollection<T>, D:Dim>{
    data: A,
    shape: Shape<D>,
    dummy: PhantomData<T>,
}
impl<T, A:MatCollection<T>> Matrix<T,A,Vec<usize>>{

    
    ///returns shape of the matrix
    pub fn get_shape(&self) -> &Shape<Vec<usize>>{
        &self.shape
    }

    
    /// returns the dimensions of the matrix as a slice.
    pub fn get_dim(& self) -> & Vec<usize> {
        &self.shape.get_dim()
    }
    /// returns thre dimensions of the matrix as a mutable slice.
    fn get_dim_mut(&mut self) -> & mut Vec<usize> {
        self.shape.get_dim_mut()
    }

    /// returns a reference to the matrices raw_data. 
    pub fn raw_data(&self) -> &A{
        &self.data 
    }
    pub fn into_raw_data(self) -> A{
        self.data
    }
    /// returns a mutable reference to the matrices raw_data.
    pub fn raw_data_mut(&mut self) -> &mut A{
        &mut self.data
    }
    pub fn get_column(&self) -> MatrixView<T>{
        unimplemented!()
    }
    
}

impl<'a,T> OwnedMatrix<T>{
    /// constructor method for owned arrays.
    pub fn new(data:Vec<T>,dim:Vec<usize>) -> OwnedMatrix<T> {
        
        let shape = Shape::new(0,dim);
        
        Matrix{data:data , shape:shape,dummy:PhantomData}
        
    }

    /// Get a view of the Data in owned OwnedMatrix
    // Should eventually be called in Deref, as soon as I figure out how
    pub fn into_view(&self)->MatrixView<'a,T>{
        unimplemented!()
    }
    ///appends a column to a matrice from raw_data.
    pub(crate) fn append_column_from_raw(&mut self, column: &mut Vec<T>) {
        
        self.shape.check_special_cases(column.len());
        self.shape.increase_size(1,1); 
        
        self.data.append(column);
        
    }
    pub fn slice(&self, limits: SliceParameters) -> MatrixView<'a,T>{
        for limit in limits {
             
        }
        unimplemented!()

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
mod tests;
    
    
        
   
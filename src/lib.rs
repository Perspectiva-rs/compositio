



#[macro_use] mod macros;
mod trait_impl;
mod dim_traits;

use dim_traits::Dim;


///An n dimensional array.
/// The array is a general container of data.
/// The array can grow.
/// The array can be indexed by using a number of coordinates equal to the dimension of the matrix.
#[derive(Debug,PartialEq)]
pub struct Matrix<T,D:  ?Sized>{
    pub data: Vec<T>,
    pub dim: D,
}
impl<'a,T> Matrix<T,Vec<usize>>{

    /// constructor method for owned arrays.
    pub fn new(data:Vec<T>,dim:Vec<usize>) -> Matrix<T,Vec<usize>> {
        Matrix{data , dim}
    }
    /// returns the dimensions of the matrix as a slice.
    pub fn dim(&'a self) -> &[usize] {
        &self.dim 
    }
    /// returns thre dimensions of the matrix as a mutable slice.
    fn mut_dim(& mut self) -> & mut [usize] {
        &mut self.dim
    }

    /// returns a reference to the matrices raw_data. 
    pub fn raw_data(&self) -> &Vec<T>{
        &self.data 
    }
    /// returns a mutable reference to the matrices raw_data.
    pub fn raw_data_mut(&mut self) -> &mut Vec<T> {
        &mut self.data
    }
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
type Matrixf32 = Matrix<f32,Vec<usize>>;
impl Matrixf32{
    /// creates matrix of with dimensions dim filled with zeros.
    pub fn zeros(dim:Vec<usize>) -> Matrixf32{
        let length = dim.iter().product();

        Matrix{data: vec![0.0;length], dim:dim}
    }
    ///creates matrix with dimensions dim filled with ones.
     pub fn ones(dim:Vec<usize>)-> Matrixf32{
        let length = dim.iter().product();

        Matrix{data: vec![1.0;length], dim:dim}
    }
}
type Matrixi32 = Matrix<i32,Vec<usize>>;
impl Matrixi32{
    /// creates matrix of with dimensions dim filled with zeros.
     pub fn zeros(dim:Vec<usize>)-> Matrixi32{
        let length = dim.iter().product();

        Matrix{data: vec![0;length], dim:dim}
    }
    ///creates matrix with dimensions dim filled with ones.
     pub fn ones(dim:Vec<usize>)-> Matrixi32{
        let length = dim.iter().product();
        Matrix{data: vec![1;length], dim:dim}
    }
}






#[cfg(test)]


mod tests {
    
    use super::*;
    
    
    //use super::macros;
    #[test]
    fn vector() {
        let matrix = Matrix::new(vec![2,3,4],vec![3, 1]);
        let macro_matrix = mat![2,3,4];
        println!("\n{}",matrix);
        println!("{},{}",matrix[&[1,0]],matrix[&[2,0]]);
        

        assert_eq!(matrix, macro_matrix);
        //assert_eq!(matrix.dim(), macro_matrix.dim());

    }
    #[test]
    fn matrix() {
        let matrix = Matrix::new(vec![2,3,4,5,6,7,8,9,10],vec![3, 3]);
        let macro_matrix = mat![2,3,4;5,6,7;8,9,10];
        
        assert_eq!(matrix, macro_matrix);
       // assert_eq!(matrix.data, macro_matrix.data);

    }
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

}

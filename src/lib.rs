use std::fmt;
use std::ops::Index;
use std::borrow::Borrow;
use std::borrow::BorrowMut;
use std::process;
pub trait Coord{
    type Dimension: Dim + ?Sized;
    fn index_checked(&self, dim: &Self::Dimension) -> Result<usize,&'static str>;

}

impl Coord for [usize]{
    type Dimension =  [usize];

    fn index_checked(&self, dim: &[usize]) ->Result<usize,&'static str>{
        
        if dim.check_bounds(&self) {
            let mut index: usize = self[0];
            for  i in 1..dim.len(){
                let dimensions_crossed: usize = dim[0..i].iter().product();
                index += self[i] * dimensions_crossed;
            }
            Ok(index)
        }else{
            Err("Index exceeds matrix dimensions")
        }
    }
}

pub trait Dim{
    type Coord: Coord + ?Sized;
    fn as_array(&self) -> &[usize];
    fn as_mut_array(&mut self) -> &mut [usize];
    fn increase_size(&mut self, axis: usize, inc: usize);
    fn get(&self,axis:usize) -> usize;
    fn set(&mut self,axis:usize,new_dim:usize);
    fn check_bounds(&self, coord: &Self::Coord)->bool;
    fn is_empty(&self) ->bool;
    fn is_column_vector(&self) -> bool;
}

impl Dim for [usize] {
    type Coord = [usize];
    
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
    fn set(&mut self,axis:usize,new_dim:usize){
        self[axis] = new_dim;
    }
    //needs to be expanded to N dimensions;
    fn check_bounds(&self, coord: &Self::Coord)->bool{
        if self.len() != coord.len(){
            false
        }else{
            let mut inbounds = true;
            for n in 0..coord.len() {
                if self[n] > coord[n]{
                    inbounds = true;
                } else{
                    inbounds = false;
                    break;
                } 
            }
            inbounds
        }
    }
    
    fn is_empty(&self) ->bool{
        if &[0usize,0usize] == self {
            true
        }else{
            false
        }
    }
        fn is_column_vector(&self) -> bool{
            if self[0] > 1 && self[1] == 0{
                true
            }else{
                false
            }
        }
}

#[derive(Debug,PartialEq)]
pub struct Matrix<T,D:  ?Sized>{
    pub data: Vec<T>,
    pub dim: D,
}
impl<'a,T> Matrix<T,Vec<usize>>{
    pub fn new(data:Vec<T>,dim:Vec<usize>) -> Matrix<T,Vec<usize>> {
        Matrix{data , dim}
    }
    pub fn dim(&'a self) -> &[usize] {
        &self.dim 
    }
    fn mut_dim(& mut self) -> & mut [usize] {
        &mut self.dim
    }

    pub fn data(&self) -> &Vec<T>{
        &self.data 
    }
    pub fn data_mut(&mut self) -> &mut Vec<T> {
        &mut self.data
    }

    pub fn append_column(&mut self, column: &mut Vec<T>) {
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

     pub fn zeros(dim:Vec<usize>) -> Matrixf32{
        let length = dim.iter().product();

        Matrix{data: vec![0.0;length], dim:dim}
    }
     pub fn ones(dim:Vec<usize>)-> Matrixf32{
        let length = dim.iter().product();

        Matrix{data: vec![1.0;length], dim:dim}
    }
}
type Matrixi32 = Matrix<i32,Vec<usize>>;
impl Matrixi32{

     pub fn zeros(dim:Vec<usize>)-> Matrixi32{
        let length = dim.iter().product();

        Matrix{data: vec![0;length], dim:dim}
    }
     pub fn ones(dim:Vec<usize>)-> Matrixi32{
        let length = dim.iter().product();

        Matrix{data: vec![1;length], dim:dim}
    }
}

impl<'a,'b, T> Index<&'a [usize]> for Matrix<T,Vec<usize>>
    

    {
    type Output = T;    
    fn index(& self, index:&[usize]) -> &T{
        let data = self.data();
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
// [1, 1, 1, 1, 1] => Vec<>
// [1, 1, 1, 1, 1]
// [1,1,1,1;1,1,1,1;1,1,1,1] 1,1,1,1,1; mat!(mat1|mat2)
#[allow(unused_macros)]
macro_rules! mat {
        ($($($x:expr),*);*) => {
            {
                let mut matrix = Matrix::new(vec![],vec![0, 0]);
                $(let mut vector = vec![$($x),*];
                matrix.append_column(&mut vector);
                );*

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
        let matrix = Matrix::new(vec![2,3,4],vec![3 as usize, 1 as usize]);
        let macro_matrix = mat![2,3,4];
        println!("\n{}",matrix);
        println!("{},{}",matrix[&[1,0]],matrix[&[2,0]]);
        

        assert_eq!(matrix, macro_matrix);
        //assert_eq!(matrix.dim(), macro_matrix.dim());

    }
    #[test]
    fn matrix() {
        let matrix = Matrix::new(vec![2,3,4,5,6,7,8,9,10],vec![3 as usize, 3 as usize]);
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
        
        assert_eq!(8,matrix[&[1,1,1,1]]);
    }
    #[test]
    fn zeros_and_ones_matrix(){
        let ones = Matrixf32::ones(vec![10,10]);
        let zeros = Matrixf32::zeros(vec![10,10]);
        println!("\n{}\n{}",ones, zeros);
    }

}

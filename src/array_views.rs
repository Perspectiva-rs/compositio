use std::ops::Index;
use super::*;

//type definitions
pub type MatrixView<'a,T> =  Matrix<T, BorrowedData<'a,T>,Vec<usize>>;

impl<'a,T> MatrixView<'a,T>{
    pub fn new_view(matrix:&'a OwnedMatrix<T>) -> MatrixView<'a,T>{
        let data = BorrowedData(&matrix.raw_data());
        let mut shape = matrix.get_shape().clone();
        shape = shape.clone();
        
        let dummy = PhantomData;
        MatrixView{data,shape,dummy}
    }
}

#[derive(Debug,PartialEq)]
pub struct BorrowedData<'a,T:'a>(&'a[T]);
impl<'a, T> MatCollection<T> for BorrowedData<'a,T>{}

impl<'a, T> Index<usize> for BorrowedData<'a, T> {
    type Output = T;

    
    fn index(&self, index: usize) -> &T {
        &self.0[index]
    }
}




#[derive(Debug,PartialEq)]
pub struct SliceParameters{
    parameters: Vec<[usize;2]>
}
impl SliceParameters{
    pub fn new(parameters:Vec<[usize;2]>) -> SliceParameters{
        if SliceParameters::check_validity(&parameters){
            SliceParameters{parameters}
        }else{
            panic!();
        }
    }
    pub fn get_dim(&self,dim: usize) -> [usize;2]{
        self.parameters[dim]
    }
    fn check_validity(parameters: &Vec<[usize;2]>) -> bool{
        let mut valid = false;
        for parameter in parameters.iter(){
            if parameter[0] > parameter[1]{
                valid = false;
            }else{
                valid = true;
            }
        }
        valid
    }
}

impl IntoIterator for SliceParameters{
    
    type Item = [usize;2];
    type IntoIter = ::std::vec::IntoIter<[usize;2]>;

    fn into_iter(self) -> Self::IntoIter {
        self.parameters.into_iter()
    }
}


#[cfg(test)]
mod test_array_views{
    use super::*;
 
}
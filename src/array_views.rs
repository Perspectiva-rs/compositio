use std::ops::Index;
use dim_traits::Coord;
use super::*;

//type definitions

pub type MatrixView<'a,T> =  BaseMatrix<BorrowedCollection<'a,T>,Vec<usize>>;

impl<'a,T> MatrixView<'a,T>{
    pub fn new(data:  &'a[T], offset:usize,stride:&Vec<usize>,dim: &Vec<usize>) ->Self{
        println!("stride MatrixView::new = {:?}", stride );
        let matrix_data = BorrowedCollection(data);
        MatrixView{data:matrix_data,shape: Shape::new(offset,stride.clone(),dim.clone())}

    }
}

#[derive(Debug,PartialEq)]
pub struct BorrowedCollection<'a,T:'a>(&'a[T]);
impl<'a, T:'a + fmt::Display > Data for BorrowedCollection<'a, T>{
    type Elem = T;
}

impl<'a, T> Index<usize> for BorrowedCollection<'a, T> {
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
    pub fn get_ndims(&self)-> usize{
        self.parameters.len()
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
    pub fn get_slice_dim(&self) -> Vec<usize>{
        let mut dim = vec![0;self.parameters.len()];
        let mut i = 0;
        for parameter in self.parameters.iter(){
            dim[i] = parameter[1] - parameter[0] + 1;   
            i += 1;        
        }
        dim
    }
    pub fn get_slice_offset(&self, shape: &Shape<Vec<usize>>) -> usize{
        let sub = self.get_first_sub();
        sub.index_checked(shape)
    }
    pub fn get_first_sub(&self) ->Vec<usize>{
        let mut sub = vec![0;self.parameters.len()];
        let mut i = 0;
        for parameter in self.parameters.iter(){
            sub[i] = parameter[0];   
            i += 1;        
        }
        sub
    }
}

impl<'a> IntoIterator for &'a SliceParameters{
    
    type Item = &'a[usize;2];
    type IntoIter = ::std::slice::Iter<'a, [usize;2]>;

    fn into_iter(self) -> Self::IntoIter {
        self.parameters.iter()
    }
}


#[cfg(test)]
mod test_array_views{
   
 
}
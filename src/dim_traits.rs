use std::borrow::Borrow;
use std::borrow::BorrowMut;

///Vectors or slices that can be used to carry coordinates to index an array.
pub trait Coord{
    type Dimension: Dim + ?Sized;
    /// function that converts the received coordinates to a column major index
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
/// Vectors or slices that can be used to identify the dimension of the matrix.
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

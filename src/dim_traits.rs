use std::borrow::Borrow;
use std::borrow::BorrowMut;
use std::ops::Index;
use std::ops::IndexMut;
use std::ops::Range;
use shape::Shape;


///Vectors or slices that can be used to carry coordinates to index an array.
pub trait Coord{
    type Dimension: Dim;
    /// function that converts the received coordinates to a column major index
    fn index_checked(&self, shape: &Shape<Self::Dimension>) -> usize;
}
impl Coord for [usize]{
    type Dimension =  Vec<usize>;

    fn index_checked(&self, shape: & Shape<Self::Dimension>) -> usize{
        if shape.get_dim().check_bounds(&self) {
            shape.sub_2_index_unchecked(self)
        }else{
            panic!("Index out of bounds");
        }
    }
}
/// Vectors or slices that can be used to identify the dimension of the matrix.
pub trait Dim:  Index<usize, Output = usize> + IndexMut<usize> + Index<Range<usize>, Output = [usize] > {
    type Coord: Coord + ?Sized;
    fn increase_axis(&mut self, axis: usize, inc: usize);
    fn n_dims(&self) -> usize;
    
    fn check_bounds(&self, coord: &Self::Coord)->bool;
    fn is_empty(&self) ->bool;
    fn is_column_vector(&self) -> bool;
}

impl Dim for Vec<usize> {

    type Coord = [usize];

    fn increase_axis(&mut self, axis: usize, inc: usize){
        
        if self.len() < axis + 1{
        
            self.push(0);

        }
        self[axis] = self[axis] + inc;
    }
    fn n_dims(&self) ->usize{
        self.len()
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
        unimplemented!()
    }
    fn is_column_vector(&self) -> bool{
        unimplemented!()
    }
}

// impl Dim for [usize] {
//     type Coord = [usize];
    
   
//     fn increase_size(&mut self, axis: usize, inc: usize){
//         self[axis] = self[axis] + inc;
//     }
//     fn n_dims(&self) ->usize{
//         self.len()
//     }
//     fn check_bounds(&self, coord: &Self::Coord)->bool{
//         if self.len() != coord.len(){
//             false
//         }else{
//             let mut inbounds = true;
//             for n in 0..coord.len() {
//                 if self[n] > coord[n]{
//                     inbounds = true;
//                 } else{
//                     inbounds = false;
//                     break;
//                 } 
//             }
//             inbounds
//         }
//     }
    
//     fn is_empty(&self) ->bool{
//         if &[0usize,0usize] == self {
//             true
//         }else{
//             false
//         }
//     }
//     fn is_column_vector(&self) -> bool{
//         if self[0] > 1 && self[1] == 0{
//             true
//         }else{
//             false
//         }
//     }
// }

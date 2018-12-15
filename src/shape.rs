
use dim_traits::Dim;

///A struct containing all the data needed to correctly index the matrix.
#[derive(Debug,PartialEq)]
pub struct Shape<D:Dim>{
    offset: usize,
    stride:D,
    dim: D,
}
impl Clone for Shape<Vec<usize>>{
    fn clone(&self) -> Self{
        Shape::new(self.offset,self.get_dim().clone())
    }
}
impl Shape<Vec<usize>>{
    pub fn new(offset: usize, dim:Vec<usize>) -> Self{

        let mut stride = vec![0;dim.len()];
        stride[0] = 1;
        for  i in 1..dim.n_dims(){
                let dimensions_crossed: usize = dim[0..i].iter().product();
                stride[i] = dimensions_crossed;
            }
        Shape{offset,dim,stride}
    }

    
    pub fn get_dim(&self) -> &Vec<usize> {
        &self.dim
    }
    pub fn get_dim_mut(& mut self) -> &mut Vec<usize> {
        &mut self.dim
    }
    pub fn is_empty(&self) -> bool{
        let sum: usize = self.dim.iter().sum(); 
        sum == 0
    }
    pub fn sub_2_index_unchecked(&self,sub:&[usize]) -> usize{
        sub.into_iter().zip(&self.stride).map(|(x, y)|{ 
            let number = x*y; 
            number} ).sum()
    }
    pub(crate) fn check_special_cases(&mut self, length_column: usize){
        if self.is_empty() {
            self.increase_size(0,length_column);
        }else{
            if length_column != self.get_dim()[0]{
                    panic!("Matrix columns have variable length");
            }
        }
    }
     pub fn increase_size(&mut self, axis:usize,amount:usize){
        if self.dim.len() < axis + 1{
            let new_axis_stride: usize = self.dim[0..axis].iter().product();
            self.stride.increase_axis(axis,new_axis_stride);
        }
        &mut self.dim.increase_axis(axis,amount);
        if self.stride.len() > axis + 1{
            &mut self.stride.increase_axis(axis + 1,amount);
        }
        
        
        
    }

}
   
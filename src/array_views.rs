pub struct SliceParameters{
    parameters: Vec<[usize;2]>
}
impl SliceParameters{
    pub fn get_dim(&self,dim: usize) -> [usize;2]{
        self.parameters[dim]
    }
}

impl IntoIterator for SliceParameters{
    
    type Item = [usize;2];
    type IntoIter = ::std::vec::IntoIter<[usize;2]>;

    fn into_iter(self) -> Self::IntoIter {
        self.parameters.into_iter()
    }
}
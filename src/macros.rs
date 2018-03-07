

///Preferred way to initiate smaller matrices.
///Numbers in a column are separated by commas. Different columns are separated by semi-colons. 
#[allow(unused_macros)]
macro_rules! mat {
    ($($($x:expr),*);*) => {
        {
            let mut matrix = Matrix::new(vec![],vec![0, 0]);
            $(let mut vector = vec![$($x),*];
            matrix.append_column_from_raw(&mut vector);
            );*

            matrix
        }
    };
}


///Preferred way to initiate smaller matrices.
///Numbers in a column are separated by commas. Different columns are separated by semi-colons. 
#[allow(unused_macros)]
macro_rules! mat {
    ($($($x:expr),*);*) => {
        {   println!("Pineapple?");
            let mut matrix = Matrix::new();
            println!("Pineapple matrix");
            $(let mut vector = vec![$($x),*];
            matrix.append_column_from_raw(&mut vector);
            );*

            matrix
        }
    };
}
#[allow(unused_macros)]
macro_rules! s {
    ($(($b:expr , $e:expr)),*) => {
        {
            let mut vector = vec![];
            $(let array = [$b,$e];
            vector.push(array);
            );+
            SliceParameters::new(vector)
        }
    };
}
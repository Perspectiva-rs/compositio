use super::*;
    
    
    //use super::macros;
    #[test]
    fn display() {
        let matrix = mat![2,3,4;5,6,7;8,9,10];
        println!("{}",matrix);
    }

    #[test]
    fn indexing() {
        let matrix = mat![2,3,4;5,6,7;8,9,10];
        println!("{:?}",matrix.get_shape());
        assert_eq!(5, matrix[&[0,1]]);
    }
    
    #[test]
    fn indexing_3d() {
        let matrix = Matrix::from_raw(vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18],vec![3,3,2]);
        assert_eq!(3,matrix[&[2,0,0]]);
        assert_eq!(18,matrix[&[2,2,1]]);
    }

    #[test]
    fn indexing_5d() {
        let matrix = Matrix::from_raw(vec![1,2,3,4, 5,6,7,8,  9,10,11,12, 13,14,15,16],vec![2,2,2,2]);
        
        assert_eq!(16,matrix[&[1,1,1,1]]);
    }
    #[test]
    fn zeros_and_ones_matrix(){
        let vector = vec![10,10];
        let ones = Matrix::<f32>::ones(vector);
        let zeros = Matrix::<f32>::zeros(vec![10,10]);
    }
    #[test]
    fn test_slice_parameter_macro(){

        let new_slice_parameter = SliceParameters::new(vec![[0,3],[0,0]]);
        let slice_parameter_macro = s!((0,3),(0,0));
        assert_eq!(slice_parameter_macro,new_slice_parameter);
    }
    #[test]
    fn test_slicing(){
        let matrix = mat![1,2,3;4,5,6];
        let view = matrix.slice(s![(0,3),(0,0)]);
    }
    #[test]
    fn test_slicing_parameter_check(){
        let matrix = mat![1,2,3;4,5,6;7,8,9];
        let view = matrix.slice(s![(0,2),(0,2)]);
        println!("{:?}",view.get_dim());
        assert_eq!(matrix.get_dim(), view.get_dim());
    }
    
    #[test]
    fn test_slice_indexing(){
        println!("Super Pineapple");
        let matrix = mat![0,1,2;3,4,5;6,7,8];
        
        let slice = matrix.slice(s![(1,1),(0,2)]);
        println!("{}", matrix);
        println!("{}",slice);
        println!("matrix[0,1]{} slice[0,0]{}", matrix[&[0,1]], slice[&[0,0]]);
        assert_eq!(matrix[&[1,0]],slice[&[0,0]]);
    }
    
    #[test]
    fn test_slice_printing(){
        let matrix = mat![0,1,2;3,4,5;6,7,8];
        let slice = matrix.slice(s![(1,1),(0,2)]);
        let another_slice = matrix.slice(s!{(0,2),(1,2)});
        println!("{}", matrix);
        println!("{}",slice);
        println!("{}",another_slice);
    }
    #[test]
    fn test_mut_slice(){
        let mut matrix = mat![0,1,2;3,4,5;6,7,8];
        
        {let mut view = matrix.as_view_mut();
        
            view[&[0,0]] = 1;
            println!("{}", view );
        }
        println!("{}", matrix);

    }
    #[test]
    fn test_numeric_support(){

        let matrix = Matrix::<f32>::zeros_(vec![5,5]);
        println!("{}", matrix);

    }
    

    // #[test]
    // fn vector_slice_test(){
    //     let matrix = mat![1,2,3;4,5,6];
    //     let slice = matrix.get_column(0);
    //     let matrix_test = mat![1,2,3];
    //     assert_eq!(slice, matrix_test);
    // }
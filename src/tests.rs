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
        let matrix = Matrix::new(vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18],vec![3,3,2]);
        assert_eq!(3,matrix[&[2,0,0]]);
        assert_eq!(18,matrix[&[2,2,1]]);
    }

    #[test]
    fn indexing_5d() {
        let matrix = Matrix::new(vec![1,2,3,4, 5,6,7,8,  9,10,11,12, 13,14,15,16],vec![2,2,2,2]);
        
        assert_eq!(16,matrix[&[1,1,1,1]]);
    }
    #[test]
    fn zeros_and_ones_matrix(){
        let vector = vec![10,10];
        let ones = Matrixf32::ones(vector);
        let zeros = Matrixf32::zeros(vec![10,10]);
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
        let test_view = mat![1,2,3].into_view();
        assert_eq!(view,test_view);
    }
    #[test]
    fn test_slicing_parameter_check(){
        
    }
    
    

    // #[test]
    // fn vector_slice_test(){
    //     let matrix = mat![1,2,3;4,5,6];
    //     let slice = matrix.get_column(0);
    //     let matrix_test = mat![1,2,3];
    //     assert_eq!(slice, matrix_test);
    // }
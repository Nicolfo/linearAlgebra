use gpurs::gpu::{MemoryCalculator, QuickCalculator};
use gpurs::linalg::Matrix;
//faer lib
use faer::{Mat};
fn main() {
    type P = f64;
    let matrix_size = 2000;
    let mut calc = QuickCalculator::<P>::init().unwrap();
    let start = std::time::Instant::now();
    let matrix: Vec<P> = (0..matrix_size * matrix_size).map(|x| x as P).collect();
    let matrix2: Vec<P> = (0..matrix_size * matrix_size).map(|x| x as P).collect();


    let temp_matrix1: Matrix<P> = Matrix::new(matrix.clone(), matrix_size, matrix_size).unwrap();
    let stored_mat1 = calc.store_matrix(temp_matrix1).unwrap();

    let temp_matrix2: Matrix<P> = Matrix::new(matrix2.clone(), matrix_size, matrix_size).unwrap();
    let stored_mat2 = calc.store_matrix(temp_matrix2).unwrap();
    let result_in_memory = calc.mat_mul(
        stored_mat1,
        stored_mat2,
    ).unwrap();


    let duration = start.elapsed();
    println!("{:?}", duration);


    let start = std::time::Instant::now();
    let temp_matrix3: Matrix<P> = Matrix::new(matrix.clone(), matrix_size, matrix_size).unwrap();
    let temp_matrix4: Matrix<P> = Matrix::new(matrix.clone(), matrix_size, matrix_size).unwrap();
    let result_quick = calc.quick_mat_mul(
        &temp_matrix3,
        &temp_matrix4,
    ).unwrap();
    let duration = start.elapsed();

    //compare the result
    // for i in 0..matrix_size {
    //     for j in 0..matrix_size {
    //         println!("{:?} {:?}", result_quick.lindex(i * matrix_size + j), result_in_memory.lindex(i * matrix_size + j));
    //         assert_eq!( result_quick.lindex(i * matrix_size + j), result_in_memory.lindex(i * matrix_size + j));
    //     }
    // }

    println!("{:?}", duration);
    // //cpu calculation
    // //fear mat from matrix
    let faer_mat1 = Mat::from_fn(matrix_size, matrix_size, |i, j| matrix[i * matrix_size + j]);
    let fear_mat2 = Mat::from_fn(matrix_size, matrix_size, |i, j| matrix2[i * matrix_size + j]);
    let start = std::time::Instant::now();


    let cpu_result = faer_mat1 * fear_mat2;
    let duration = start.elapsed();

    println!("{:?}",duration);

    for i in 0..matrix_size {
        for j in 0..matrix_size {
          //  println!("{:?} {:?}", cpu_result.get(i,j), result_in_memory.lindex(i * matrix_size + j));
            assert_eq!( *cpu_result.get(i,j), result_quick.lindex(i * matrix_size + j));
        }
    }

    println!("result are equals")


}

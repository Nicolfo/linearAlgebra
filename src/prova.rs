use faer::{MatRef};
use faer::mat::{from_column_major_slice, from_row_major_slice};
use faer::prelude::SpSolver;

#[no_mangle]
pub extern "C" fn solve(
    size: usize,
    a: *const f64,
    b: *const f64,
    x: *mut f64,
) {
    let rows = unsafe { std::slice::from_raw_parts(a, size*size)};
    let mat: MatRef<f64> = from_row_major_slice(rows, size, size);
    let b = unsafe { std::slice::from_raw_parts(b,size)};
    let b: MatRef<f64> = from_column_major_slice(b, size,1);
    let res = unsafe { std::slice::from_raw_parts_mut(x, size)};
    let x = mat.partial_piv_lu().solve(&b);
    for i in 0..size {
        res[i] = x.read(i,0);
    }
    /*println!("======== RUST ========");
    println!("A: {:?}", mat);
    println!("B: {:?}", b);
    println!("X: {:?}", x);
    println!("======================")*/
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let result = 4;
        assert_eq!(result, 4);
    }
}
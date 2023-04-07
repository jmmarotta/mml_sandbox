use ndarray::prelude::*;

// this method not very *rustacean*, but helps to illustrate which index is used
fn gaussian_elimination(a: Array2<f64>) -> Array2<f64> {
    // need to check if invertible?
    let mut a = a;

    let mut pivot = (0, 0);
    for i in 0..a.ncols() - 1 {
      println!("{:?}", a);
      println!("{:?}", (pivot.0..a.nrows()).collect::<Vec<usize>>());

      let all_zeros = (pivot.0..a.nrows()).collect::<Vec<usize>>().iter().all(|&j| a[[j, pivot.1]] == 0.);
      if all_zeros {
        pivot.1 += 1;
        continue;
      }
      // rearrage rows so the largest number in the col is used as a pivot
      for j in pivot.0..a.nrows() {
        if pivot.0 == j {
          continue;
        } else {
          if a[[j, pivot.1]] > a[[pivot.0, pivot.1]] {
            // swap the rows
            for k in 0..a.ncols() {
              let tmp = a[[pivot.0, k]];
              a[[pivot.0, k]] = a[[j, k]];
              a[[j, k]] = tmp;
            }
          }
        }
      }
      println!("{:?}", pivot);
      // TODO: should iterate with an iterable of coordinates
      for j in 0..a.nrows() {
        let multiplier = a[[j, pivot.1]] / a[[pivot.0, pivot.1]];
        for k in 0..a.ncols() {
          if j == pivot.0 { continue; }
          let num = a[[j, k]] - multiplier * a[[pivot.0, k]];
          println!("{:?} - {:?} = {:?}", a[[j,k]], multiplier * a[[pivot.0, k]], num);
          a[[j, k]] -= multiplier * a[[pivot.0, k]]
        }
      }
      pivot.0 += 1;
      pivot.1 += 1;
    }

    return a;
}

fn main() {
    println!("Hello, world!");

    let a = array![
                [1.,0.,1., 2.], 
                [4.,0.,3., 7.],
                [1.,2.,-1., 4.]
            ];

    let r1 = gaussian_elimination(a);
    
    println!("{:?}", r1);

    let b = array![
      [2.,2.,6., 2., 14.], 
      [4.,4.,0., -1., 11.],
      [0.,0.,6., -1., 5.]
    ];

    let r2 = gaussian_elimination(b);
    
    println!("{:?}", r2);

}

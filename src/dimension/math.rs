use crate::print2;

use super::Point::Float;

pub fn mat_mul<
    const N: usize,
    const M: usize,
    const I: usize,
    T: std::ops::AddAssign<<T as std::ops::Mul>::Output> + Default + Copy + std::ops::Mul,
>(
    m1: [[T; M]; I],
    m2: [[T; N]; M],
) -> [[T; N]; I] {
    let mut target: [[T; N]; I] = [[Default::default(); N]; I];
    for i in 0..I {
        for n in 0..N {
            let m1 = &m1[i];
            let mut res = Default::default();
            for m in 0..M {
                res += m1[m] * m2[m][n];
            }
            target[i][n] = res;
        }
    }
    return target;
}

#[test]
pub fn test_mat_mul() {
    {
        let mat1 = [[1, 2, 3], [4, 5, 6]];
        let mat2 = [[10], [11], [12]];
        let res = mat_mul(mat1, mat2);
        assert_eq!(res,[[68],[167]]);
    }

    {
        let mat1 = [[5,3,4,0], [2,-1,1,6]];
        let mat2 = [[5,3,7,-4],[0,4,8,1],[6,2,9,10],[11,12,13,-2]];
        let res = mat_mul(mat1, mat2);
        assert_eq!(res,[[49,35,95,23],[82,76,93,-11]]);
    }

}

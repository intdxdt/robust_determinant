extern crate robust_sum;
extern crate robust_scale;
extern crate two_product;
extern crate robust_compress_seq;

use robust_compress_seq::compress;
use robust_sum::robust_sum as rsum;
use robust_scale::robust_scale as rscale;
use two_product::two_product as tprod;

pub fn det2(m: &[[f64;2]]) -> Vec<f64> {
    let mut e = rsum(
        &tprod(m[0][0], m[1][1]),
        &tprod(-m[0][1], m[1][0])
    );
    compress(&mut e)
}

pub fn det3(m: &[[f64;3]]) -> Vec<f64> {
    let mut e = rsum(
        &rscale(&rsum(
            &tprod(m[1][1], m[2][2]),
            &tprod(-m[1][2], m[2][1])
        ), m[0][0]),
        &rsum(
            &rscale(
                &rsum(
                    &tprod(m[1][0], m[2][2]),
                    &tprod(-m[1][2], m[2][0])
                ), -m[0][1]),
            &rscale(
                &rsum(
                    &tprod(m[1][0], m[2][1]),
                    &tprod(-m[1][1], m[2][0])
                ), m[0][2]
            ),
        ),
    );
    compress(&mut e)
}

#[cfg(test)]
mod test_det {
    use super::{det2, det3};

    #[test]
    fn test_robust_det() {
        assert_eq!(det3(&[[1., 2., 3.], [3., 4., 5.], [6., 7., 8.]]), [0.]);
        assert_eq!(det2(&[[1., 2.], [3., 4.]]), [-2.]);
        assert_eq!(det3(&[[1., 2., 3.], [3., 4., 5.], [6., 7., 8.]]), [0.]);
        assert_eq!(det2(&[[1., 2.], [3., 4.]]), [-2.]);
    }
}
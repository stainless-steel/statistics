use {Real, ToReal};

/// Compute an estimate of the population mean from a finite sample.
#[inline]
pub fn mean<T: ToReal<R>, R: Real>(data: &[T]) -> R {
    data.iter().fold(R::zero(), |sum, x| sum + x.to_real()) / R::natural(data.len())
}

/// Compute an estimate of the population variance from a finite sample.
///
/// The estimate is unbiased. The computation is based on the
/// compensated-summation version of the [two-pass algorithm][1].
///
/// [1]: https://en.wikipedia.org/wiki/Algorithms_for_calculating_variance#Two-pass_algorithm
pub fn variance<T: ToReal<R>, R: Real>(data: &[T]) -> R {
    match data.len() {
        1 => R::zero(),
        n => {
            let mu = data.iter().fold(R::zero(), |sum, x| sum + x.to_real()) / R::natural(n);
            let (mut sum1, mut sum2) = (R::zero(), R::zero());
            for x in data {
                let delta = x.to_real() - mu;
                sum1 = sum1 + delta * delta;
                sum2 = sum2 + delta;
            }
            (sum1 - sum2 * sum2 / R::natural(n)) / (R::natural(n) - R::one())
        },
    }
}

#[cfg(test)]
mod tests {
    use assert;

    #[test]
    fn mean() {
        let data = [
             5.3766713954610001e-01,  1.8338850145950865e+00,
            -2.2588468610036481e+00,  8.6217332036812055e-01,
             3.1876523985898081e-01, -1.3076882963052734e+00,
            -4.3359202230568356e-01,  3.4262446653864992e-01,
             3.5783969397257605e+00,  2.7694370298848772e+00,
        ];
        assert::close(&[super::mean(&data)], &[6.2428219709029698e-01], 1e-15);
    }

    #[test]
    fn mean_0() {
        assert!(super::mean::<f64, _>(&[]).is_nan());
    }

    #[test]
    fn variance() {
        let data = [
             5.3766713954610001e-01,  1.8338850145950865e+00,
            -2.2588468610036481e+00,  8.6217332036812055e-01,
             3.1876523985898081e-01, -1.3076882963052734e+00,
            -4.3359202230568356e-01,  3.4262446653864992e-01,
             3.5783969397257605e+00,  2.7694370298848772e+00,
        ];
        assert::close(&[super::variance(&data)], &[3.1324921339484746e+00], 1e-15);
    }

    #[test]
    fn variance_0() {
        assert!(super::variance::<f64, _>(&[]).is_nan());
    }

    #[test]
    fn variance_1() {
        assert_eq!(super::variance::<_, f64>(&[1.0]), 0.0);
    }
}

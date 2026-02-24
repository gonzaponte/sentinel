
pub fn interpolate_1d( x          : &Vec<f64>
                     , y          : &Vec<f64>
                     , x_new      : f64
                     ) -> Result<f64, &'static str> {
    if x_new < *x.first().unwrap() { return Err("x value below interpolation range")}

    let i = x.iter()
             .enumerate()
             .skip(1)
             .filter(|(_,xi)| **xi > x_new)
             .nth(0)
             .ok_or("x value above interpolation range")?
             .0;

    let result = (x_new - x[i-1]) * (y[i] - y[i-1]) / (x[i] - x[i-1]) + y[i-1];
    Ok(result)
}

pub fn linspace(min: f64, max: f64, n: usize) -> Vec<f64> {
    match n {
        0 => vec![],
        1 => vec![min],
        _ => {
            let delta = (max - min) / ( n.saturating_sub(1) as f64);

            (0..n).map(|i| min + (i as f64) * delta)
                  .collect()
        },
    }
}

pub fn digitize(value: f64, bins: &Vec<f64>) -> Option<usize> {
    if value < *bins.first().unwrap() { return None; }

    bins[1..].iter()
             .position(|lower| value < *lower)
}

#[cfg(test)]
use tempfile::{TempDir, tempdir};

#[cfg(test)]
pub fn tempfile(stem: &str) -> (TempDir, String) {
    let dir  = tempdir().unwrap();
    let file = dir.path()
                  .join(stem)
                  .to_str()
                  .unwrap()
                  .to_string();
    (dir, file)
}


#[cfg(test)]
mod tests {
    use super::*;

    use float_eq::assert_float_eq;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[test]
    fn test_interpolate_1d_simple() {
        let xp = vec![ 1.,  2.,  3.];
        let yp = vec![ 4.,  5.,  6.];
        let xn = vec![-3., -2., -1.];
        let yn = vec![-6., -5., -4.];

        let result = interpolate_1d(&xp, &yp,  1.5).unwrap(); assert_eq!(result,  4.5);
        let result = interpolate_1d(&xp, &yp,  2.5).unwrap(); assert_eq!(result,  5.5);
        let result = interpolate_1d(&xn, &yp, -2.5).unwrap(); assert_eq!(result,  4.5);
        let result = interpolate_1d(&xn, &yp, -1.5).unwrap(); assert_eq!(result,  5.5);
        let result = interpolate_1d(&xp, &yn,  1.5).unwrap(); assert_eq!(result, -5.5);
        let result = interpolate_1d(&xp, &yn,  2.5).unwrap(); assert_eq!(result, -4.5);
        let result = interpolate_1d(&xn, &yn, -2.5).unwrap(); assert_eq!(result, -5.5);
        let result = interpolate_1d(&xn, &yn, -1.5).unwrap(); assert_eq!(result, -4.5);
    }

    #[test]
    fn test_interpolate_1d_quadratic() {
        let x = vec![1., 3.,  5.];
        let y = vec![1., 9., 25.];

        let result = interpolate_1d(&x, &y, 2.).unwrap(); assert_eq!(result,  5.);
        let result = interpolate_1d(&x, &y, 4.).unwrap(); assert_eq!(result, 17.);
    }

    #[test]
    fn test_interpolate_1d_range() {
        let x = vec![-1., 1.];
        let y = x.clone(); // irrelevant

        let result = interpolate_1d(&x, &y, -2.); assert!(result.is_err()); // out
        let result = interpolate_1d(&x, &y, -1.); assert!(result.is_ok ()); // lower limit is closed
        let result = interpolate_1d(&x, &y,  0.); assert!(result.is_ok ()); // in
        let result = interpolate_1d(&x, &y,  1.); assert!(result.is_err()); // upper limit is open
        let result = interpolate_1d(&x, &y,  2.); assert!(result.is_err()); // out
    }

    #[rstest]
    #[case(1)]
    #[case(12)]
    #[case(123)]
    #[case(1234)]
    fn test_linspace_len(#[case] n: usize) {
        let v = linspace(0.0, 1.0, n);
        assert_eq!(v.len(), n);
    }

    #[test]
    fn test_linspace_basic() {
        let v = linspace(0.0, 1.0, 5);
        assert_float_eq!(v[0], 0.00, ulps<=2);
        assert_float_eq!(v[1], 0.25, ulps<=2);
        assert_float_eq!(v[2], 0.50, ulps<=2);
        assert_float_eq!(v[3], 0.75, ulps<=2);
        assert_float_eq!(v[4], 1.00, ulps<=2);
    }

    #[test]
    fn test_linspace_single_point() {
        let v = linspace(3.0, 10.0, 1);
        assert_float_eq!(v[0], 3.0, ulps<=2);
    }

    #[test]
    fn test_linspace_zero_points() {
        let v = linspace(0.0, 1.0, 0);
        assert!(v.is_empty());
    }

    #[test]
    fn test_linspace_descending() {
        let v = linspace(1.0, 0.0, 3);
        assert_float_eq!(v[0], 1.00, ulps<=2);
        assert_float_eq!(v[1], 0.50, ulps<=2);
        assert_float_eq!(v[2], 0.00, ulps<=2);
    }

    #[test]
    fn test_digitize_basic() {
        let bins = vec![-9.0, 0.0, 1.0, 8.0];

        assert_eq!(digitize(-3.5, &bins), Some(0));
        assert_eq!(digitize( 0.5, &bins), Some(1));
        assert_eq!(digitize( 7.9, &bins), Some(2));
    }

    #[test]
    fn test_digitize_exact_bin_edges() {
        let bins = vec![0.0, 1.0, 4.0, 9.0];

        assert_eq!(digitize(0.0, &bins), Some(0));
        assert_eq!(digitize(1.0, &bins), Some(1));
        assert_eq!(digitize(4.0, &bins), Some(2));
        assert!   (digitize(9.0, &bins).is_none());
    }

    #[test]
    fn test_digitize_below_first_bin() {
        let bins = vec![0.0, 2.0];
        assert!(digitize(-0.1, &bins).is_none());
    }

    #[test]
    fn test_digitize_above_last_bin() {
        let bins = vec![0.0, 3.0];
        assert!(digitize(5.0, &bins).is_none());
    }
}

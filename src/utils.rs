
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

#[cfg(test)]
mod tests {
    use super::*;

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

}

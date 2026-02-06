use std::f64::consts::TAU;

use nalgebra::Point3;
use rand::{random, rng};
use rand_distr::{Distribution, Normal};

pub fn uniform(low: f64, upp: f64) -> f64 {
    low + random::<f64>() * (upp - low)
}

pub fn normal(mu: f64, sig: f64) -> f64 {
    Normal::new(mu, sig).unwrap().sample(&mut rng())
}

pub fn circle(r: f64) -> (f64, f64) {
    let r   = uniform(0f64, r*r).sqrt();
    let phi = uniform(0f64, TAU);
    ( r * phi.cos(), r * phi.sin() )
}

pub fn in_cone(r_min: f64, form_factor: f64, z_max: f64) -> Point3<f64> {
    let z      = uniform(0f64, z_max);
    let r      = r_min + form_factor * z;
    let (x, y) = circle(r);
    Point3::new(x, y, z)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uniform_within_range() {
        let low = 123.;
        let upp = 456.;
        let range = low..upp;

        for _ in 0..100000 {
            let x = uniform(low, upp);
            assert!(range.contains(&x));
        }
    }

    #[test]
    fn test_uniform_limits_close() {
        let low = 123.;
        let upp = 456.;

        let mut xmin = upp;
        let mut xmax = low;
        for _ in 0..100000 {
            let x = uniform(low, upp);
            xmin = xmin.min(x);
            xmax = xmax.max(x);
        }
        assert!(xmin -  low < 1f64);
        assert!(upp  - xmax < 1f64);
    }

    #[test]
    fn test_normal_standard() {
        let data = (0..100_000).map(|_| normal(0., 1.)).collect::<Vec<f64>>();
        let mu   =  data.iter().sum::<f64>() / (data.len() as f64);
        let std  = (data.iter().map(|x| (*x - mu).powi(2)).sum::<f64>() / ((data.len() - 1) as f64)).sqrt();

        assert!((-0.1..0.1).contains(&mu));
        assert!(( 0.5..1.5).contains(&std));
    }

    #[test]
    fn test_normal_nonstandard() {
        let data = (0..100_000).map(|_| normal(-10., 15.5)).collect::<Vec<f64>>();
        let mu   =  data.iter().sum::<f64>() / (data.len() as f64);
        let std  = (data.iter().map(|x| (*x - mu).powi(2)).sum::<f64>() / ((data.len() - 1) as f64)).sqrt();

        assert!((-10.5..-9.5).contains(&mu));
        assert!(( 14.5..16.5).contains(&std));
    }

    #[test]
    fn test_circle_within_radius() {
        let rmax = 123.456;
        for _ in 0..100000 {
            let (x,y) = circle(rmax);
            let r = (x*x + y*y).sqrt();
            assert!(r < rmax);
        }
    }


    #[test]
    fn test_circle_limit_close() {
        let rmax = 123.456;
        let mut max = 0f64;
        for _ in 0..100000 {
            let (x,y) = circle(rmax);
            let r = (x*x + y*y).sqrt();
            max = max.max(r);
        }
        assert!(rmax -  max < 1f64);
    }

    #[test]
    fn test_in_cone_within_range_form_factor_1() {
        let rmin        = 12.3;
        let form_factor = 1.0;
        let zmax        = 67.8;
        for _ in 0..100000 {
            let p = in_cone(rmin, form_factor, zmax);
            let r = (p.x*p.x + p.y*p.y).sqrt() - rmin;
            let z = p.z;
            assert!(r <= z);
        }
    }

    #[test]
    fn test_in_cone_within_range_form_factor_100() {
        // with a large form factor, few points are expected to fall within the
        // same cone with a form factor = 1
        let rmin        = 12.3;
        let form_factor = 100.;
        let zmax        = 67.8;
        let mut n_in    = 0;
        for _ in 0..100000 {
            let p = in_cone(rmin, form_factor, zmax);
            let r = (p.x*p.x + p.y*p.y).sqrt() - rmin;
            let z = p.z;
            if r <= z {n_in += 1}
        }
        assert!(n_in < 1000);
    }
}

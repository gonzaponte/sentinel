use std::f64::consts::TAU;

use nalgebra::Point3;
use rand::{random, rng};
use rand_distr::{Distribution, Normal, Poisson, Exp};

pub fn sample() -> f64 {
    random::<f64>()
}

pub fn uniform(low: f64, upp: f64) -> f64 {
    low + random::<f64>() * (upp - low)
}

pub fn normal(mu: f64, sig: f64) -> f64 {
    Normal::new(mu, sig).unwrap().sample(&mut rng())
}

pub fn poisson(mu: f64) -> usize {
    Poisson::new(mu).unwrap().sample(&mut rng()) as usize
}

pub fn expo(scale: f64) -> f64 {
    Exp::new(scale).unwrap().sample(&mut rng())
}

pub fn multiexpo(fractions: &Vec<f64>, scales: &Vec<f64>) -> f64 {
    let scale = choice(scales, fractions);
    expo(*scale)
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

pub fn choice<'a, T>(values: &'a Vec<T>, probs: &Vec<f64>) -> &'a T {
    // probs are assumed to be cumulatively normalized
    let r = sample();
    for (v, p) in values.iter().zip(probs.iter()) {
        if r < *p {
            return v;
        }
    }
    panic!("[random::choice] probabilities are not cumulatively normalized");
}

pub fn exp_survival(t: f64, scale: f64) -> bool {
    -scale * sample().ln() > t
}

pub fn exp_survival_n(t: f64, scale: f64, n: usize) -> usize {
    (0..n).filter(|_| exp_survival(t, scale)).count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

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
    fn test_poisson_mean() {
        let data = (0..100_000).map(|_| poisson(2.5)).collect::<Vec<_>>();
        let mean = (data.iter().sum::<usize>() as f64) / (data.len() as f64);
        assert!( (-0.1..0.1).contains(&(mean - 2.5)) );
    }

    #[test]
    fn test_poisson_extremes() {
        for _ in 0..100_000 {
            let data = poisson(123.456);
            assert!(data >         0); // very very unlikely
            assert!(data < 1_000_000); // very very unlikely
        }
    }

    #[test]
    fn test_expo() {
        for _ in 0..100_000 {
            let data = expo(123.456);
            assert!(data > 0.0);
            assert!(data < 1.0); // very very unlikely
        }
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

    #[test]
    fn test_choice_exclusive() {
        let values = vec![-1, 2, i32::MIN];
        let probs  = vec![1., 0., 0.];
        for _ in 0..1000 {
            let c = choice(&values, &probs);
            assert_eq!(*c, -1);
        }

        let values = vec![-1, 2, i32::MIN];
        let probs  = vec![0., 1., 0.];
        for _ in 0..1000 {
            let c = choice(&values, &probs);
            assert_eq!(*c, 2);
        }

        let values = vec![-1, 2, i32::MIN];
        let probs  = vec![0., 0., 1.];
        for _ in 0..1000 {
            let c = choice(&values, &probs);
            assert_eq!(*c, i32::MIN);
        }
    }

    #[test]
    fn test_choice_combined() {
        let values = vec![0, 1, 2];
        let probs  = vec![0.1, 0.4, 1.0];
        let mut counts = vec![0, 0, 0];
        for _ in 0..1000 {
            let c = choice(&values, &probs);
            counts[*c] += 1;
        }
        assert!(counts[1] > counts[0]);
        assert!(counts[2] > counts[1]);
    }

    #[test]
    #[should_panic]
    fn test_choice_panics() {
        let values = vec![0, 1, 2];
        let probs  = vec![0.1, 0.2, 0.3];
        for _ in 0..100 {
            choice(&values, &probs);
        }
    }

    #[test]
     fn test_exp_survival_extreme_low() {
         for _ in 0..10000 {
             let out = exp_survival(1.0, 1e-15);
             assert!(!out);
         }
     }

    #[test]
     fn test_exp_survival_extreme_high() {
         for _ in 0..10000 {
             let out = exp_survival(1.0, 1e15);
             assert!(out);
         }
     }

}

use std::f64::consts::{PI, TAU};

use float_eq::float_eq;
use nalgebra::{Point3, Vector3, point, vector};

use crate::geometry::{Cone, Geometry};
use crate::random;

pub trait Sampler<G: Geometry>: Send + Sync {
    fn sample(&self, geometry: &G) -> Point3<f64>;
}

pub struct FixedPositionSampler(pub Point3<f64>);
pub struct PlaneSampler(pub f64); // angle
pub struct VolumeSampler;
pub struct SurfaceSampler;
pub struct EdgeVolumeSampler(pub f64); // distance_from_edge


impl<G: Geometry> Sampler<G> for FixedPositionSampler {
    fn sample(&self, _g: &G) -> Point3<f64> {
        self.0
    }
}


impl Sampler<Cone> for PlaneSampler {
    fn sample(&self, g: &Cone) -> Point3<f64> {
        // join the profile of a cone with an upside down cone to make a
        // rectangle. Sample it uniformly and reflect around the center

        let mut z = random::sample() * g.zmax;
        let mut r = random::sample() * (g.rmin + g.rmax());
        if r > g.r_at_z(z) {
            r = g.rmin + g.rmax() - r;
            z = g.zmax - z;
        }
        let p  = self.0;
        point![r*p.cos(), r*p.sin(), z]
    }
}


impl Sampler<Cone> for VolumeSampler {
    fn sample(&self, g: &Cone) -> Point3<f64> {
        let zf = g.rmin / g.rmax(); // fraction of total cone height
        let z0 = g.rmin / g.form_factor;
        let z  = random::uniform(zf.powi(3), 1.0).cbrt() * (g.zmax + z0);
        let r  = random::sample().sqrt() * g.form_factor * z;
        let p  = random::sample() * TAU;
        point![r*p.cos(), r*p.sin(), z-z0]
    }
}


impl Sampler<Cone> for SurfaceSampler {
    fn sample(&self, g: &Cone) -> Point3<f64> {
        let factor    = (1f64 + g.form_factor.powi(-2)).sqrt();
        // a is the cone side length
        // a0 is the side length of the truncated tip
        let a         = g.rmax() * factor;
        let a0        = g.rmin   * factor;
        let z0        = g.rmin / g.form_factor;
        let theta     = TAU / factor;
        let area_side = 0.5 * theta * (a*a - a0*a0);
        let area_base = PI * g.rmax().powi(2);
        let area_sum  = area_side + area_base;
        let probs     = vec![area_side/area_sum, 1f64];
        let labels    = vec![true, false];

        let sample_side = random::choice(&labels, &probs);
        let (r, z) = if *sample_side {
            let rflat  = random::uniform(a0, a);
            let z = rflat / factor;
            let r = z * g.form_factor;
            (r, z - z0)
        }
        else {
            let z = g.zmax;
            let r = random::sample().sqrt() * g.rmax();
            (r, z)
        };

        let p = random::sample() * TAU;
        Point3::new(r * p.cos(), r * p.sin(), z)
    }
}


impl Sampler<Cone> for EdgeVolumeSampler {
    fn sample(&self, g: &Cone) -> Point3<f64> {
        let mut out = point![0., 0., -1.];
        while !g.is_within(&out) {
            let p = SurfaceSampler{}.sample(g);
            let n = if float_eq!(p.z, g.zmax, ulps<=2) {
                Vector3::z()
            } else {
                vector![p.x, p.y, -g.form_factor*g.r_at_z(p.z)].normalize()
            };
            out = p - n * (random::sample() * self.0);
        }
        out
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use float_eq::float_eq;
    use crate::assert_point3_eq;

    #[test]
    fn test_fixed_position_sampler() {
        let dummy   = Cone::new(0., 1., 2.);
        let p0      = point![9., 8., 7.];
        let sampler = FixedPositionSampler(p0.clone());
        for _ in 0..100 {
            let p = sampler.sample(&dummy);
            assert_point3_eq!(p, p0, ulps<=2);
        }
    }


    #[test]
    fn test_volume_sampler_cone() {
        let rmin    = 1.234;
        let zmax    = 5.678;
        let cone    = Cone{rmin, form_factor: 1., zmax};
        let sampler = VolumeSampler{};
        for _ in 0..1000 {
            let p = sampler.sample(&cone);
            let r = (p.x.powi(2) + p.y.powi(2)).sqrt();
            assert!(r - rmin < p.z);
            assert!(p.z <= zmax)
        }
    }

    #[test]
    fn test_plane_sampler_cone() {
        let rmin    = 1.234;
        let zmax    = 5.678;
        let cone    = Cone{rmin, form_factor: 1., zmax};
        let sampler = PlaneSampler(PI/2.0);
        for _ in 0..1000 {
            let p = sampler.sample(&cone);
            let r = (p.x.powi(2) + p.y.powi(2)).sqrt();
            assert!(r - rmin < p.z);
            assert!(p.z <= zmax);
            assert!(p.x.abs() < 1e-12);
        }
    }

    #[test]
    fn test_surface_sampler_cone() {
        let rmin    = 1.234;
        let zmax    = 5.678;
        let cone    = Cone{rmin, form_factor: 1., zmax};
        let sampler = SurfaceSampler{};
        for _ in 0..1000 {
            let p = sampler.sample(&cone);
            let r = (p.x.powi(2) + p.y.powi(2)).sqrt();

            assert!( float_eq!(p.z, zmax, ulps<=2) ||
                     float_eq!((r-rmin)/p.z, 1.0, abs<=1e-6)
                   );
        }
    }

    #[test]
    fn test_edge_sampler_cone() {
        let rmin    = 1.234;
        let zmax    = 5.678;
        let cone    = Cone{rmin, form_factor: 1., zmax};
        let d       = 0.00001;
        let tol     = d * 2f64.sqrt(); // given by form factor
        let sampler = EdgeVolumeSampler(d);
        for _ in 0..1000 {
            let p = sampler.sample(&cone);
            let r = (p.x.powi(2) + p.y.powi(2)).sqrt();
            let rmax = cone.r_at_z(p.z);
            assert!( cone.is_within(&p) );
            assert!( (zmax - p.z < 2.*d) || (rmax - r   < tol));
        }
    }

}

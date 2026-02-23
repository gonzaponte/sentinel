use nalgebra::Point3;

use derive_new::new;

pub trait Geometry {
    fn is_within(&self, pos: &Point3<f64>) -> bool;
    fn cathode_z(&self) -> f64;
}

#[derive(Clone, Debug, new)]
pub struct Cone {
    pub rmin       : f64,
    pub form_factor: f64,
    pub zmax       : f64,
}

impl Cone {
    pub fn rmax(&self) -> f64 {
        self.r_at_z(self.zmax)
    }

    pub fn r_at_z(&self, z: f64) -> f64 {
        self.rmin + self.form_factor * z
    }
}

impl Geometry for Cone {
    fn is_within(&self, pos: &Point3<f64>) -> bool {
        if pos.z < 0.0       { return false; }
        if pos.z > self.zmax { return false; }

        let r = (pos.x.powi(2) + pos.y.powi(2)).sqrt();
        r < self.r_at_z(pos.z)
    }

    fn cathode_z(&self) -> f64 {
        self.zmax
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_r_at_z() {
        let geo = Cone::new(1., 2., 4.);

        let range = geo.rmin .. (geo.rmin) + 2.;
        for i in 0..100 {
            let zi = (i as f64) / 100f64;
            let ri = geo.r_at_z(zi);
            assert!(range.contains(&ri));
        }
    }

    #[test]
    fn test_cone_is_within() {
        let geo = Cone::new(1., 1., 10.);

        // z values are negative, despite the geometry being defined as positive
        let p0 = Point3::<f64>::new(0., 0., 0.); // in
        let p1 = Point3::<f64>::new(2., 0., 0.); // out
        let p2 = Point3::<f64>::new(1., 0., 1.); // in
        let p3 = Point3::<f64>::new(4., 0., 1.); // out
        let p4 = Point3::<f64>::new(6., 0., 7.); // in
        let p5 = Point3::<f64>::new(8., 0., 7.); // out

        assert!( geo.is_within(&p0) );
        assert!(!geo.is_within(&p1) );
        assert!( geo.is_within(&p2) );
        assert!(!geo.is_within(&p3) );
        assert!( geo.is_within(&p4) );
        assert!(!geo.is_within(&p5) );
    }

}

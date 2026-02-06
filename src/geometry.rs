use nalgebra::Point3;

#[derive(Clone, Debug)]
pub struct Geometry {
    pub rmin       : f64,
    pub form_factor: f64,
    pub zmin       : f64,
    pub zmax       : f64,
    pub rmax       : f64,
}

impl Geometry {
    pub fn new(rmin: f64, form_factor: f64, zmin: f64, zmax: f64) -> Self {
        let rmax = rmin + form_factor * zmax;
        Self{ rmin, form_factor, zmin, zmax, rmax }
    }

    pub fn is_within(&self, pos: &Point3<f64>) -> bool {
        let z =  -pos.z;
        if z < self.zmin { return false; }
        if z > self.zmax { return false; }

        let r = (pos.x.powi(2) + pos.y.powi(2)).sqrt();
        let rmax_at_z = self.rmin + self.form_factor * z;
        r < rmax_at_z
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_within() {
        let geo = Geometry::new(1., 1., 0., 10.);

        // z values are negative, despite the geometry being defined as positive
        let p0 = Point3::<f64>::new(0., 0.,  0.); // in
        let p1 = Point3::<f64>::new(2., 0.,  0.); // out
        let p2 = Point3::<f64>::new(1., 0., -1.); // in
        let p3 = Point3::<f64>::new(4., 0., -1.); // out
        let p4 = Point3::<f64>::new(6., 0., -7.); // in
        let p5 = Point3::<f64>::new(8., 0., -7.); // out

        assert!( geo.is_within(&p0) );
        assert!(!geo.is_within(&p1) );
        assert!( geo.is_within(&p2) );
        assert!(!geo.is_within(&p3) );
        assert!( geo.is_within(&p4) );
        assert!(!geo.is_within(&p5) );
    }
}

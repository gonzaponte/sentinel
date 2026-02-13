use std::io::Result;

use nalgebra::Point3;

use sentinel::field::Field;
use sentinel::tracker::Tracker;
use sentinel::geometry::Geometry;

pub fn main() -> Result<()> {
    let geometry = Geometry::new(1., 1., 10.);
    let field    = Field::from_file("data/partial_efield.dat");
    let tracker  = Tracker::new(field, geometry, 1e-6);
    tracker.propagate_from(Point3::origin());
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::{Point2, Point3, Vector2};
    use sentinel::field_point::FieldPoint;

    fn homogeneous_field() -> Field {
        let points   = vec![
            FieldPoint::new(Point2::<f64>::new(0., 11.), 1., 0, Vector2::new(0., -1.)), // startpoint
            FieldPoint::new(Point2::<f64>::new(0.,  1.), 1., 0, Vector2::new(0., -1.)), //
            FieldPoint::new(Point2::<f64>::new(0.,  0.), 1., 0, Vector2::zeros()     ), //   endpoint
        ];
        Field::new(points)
    }

    #[test]
    fn test_it_runs() {
        let geometry = Geometry::new(1., 1., 0., 10.);
        let tracker  = Tracker::new(homogeneous_field(), geometry, 1e-2);
        let t = tracker.propagate_from(Point3::new(0., 0., -9.));
        assert!(t.len() > 100, "Track length too short: {}", t.len());
        assert!(t.last().unwrap().z > -2e-2); // close to zmin
    }

    #[test]
    fn test_it_runs_slow() {
        let geometry = Geometry::new(1., 1., 0., 10.);
        let tracker  = Tracker::new(homogeneous_field(), geometry, 5e-4);
        let t = tracker.propagate_from(Point3::new(0., 0., -9.));

        assert!(t.len() > 1000, "Track length too short: {}", t.len());
        assert!(t.last().unwrap().z > -1e-3); // close to zmin
    }
}

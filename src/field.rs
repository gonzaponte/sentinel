use std::ops::{Deref, DerefMut};
use nalgebra::{Point2, Point3};
use derive_new::new;

use crate::io::read_csv;
use crate::field_point::FieldPoint;

#[derive(Clone, Debug, new)]
pub struct Field(Vec<FieldPoint>);

impl Deref for Field {
    type Target = Vec<FieldPoint>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Field {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}


impl Field {
    pub fn from_file(filename: &str) -> Self {
        let data = read_csv(filename, " ", 8).expect("Could not read field file");

        let mut previous_pos  = Point2::<f64>::origin();
        let mut  current_line = 999_999_999_999_999usize;

        let points = data.into_iter()
                         .rev()
                         .filter_map(|row| FieldPoint::from_csv_row(row, &mut previous_pos, &mut current_line))
                         .collect();

        Field::new(points)
    }

    pub fn find_nearest(&self, pos: &Point3<f64>) -> &FieldPoint {
        let r  = (pos.x*pos.x + pos.y*pos.y).sqrt();
        let p0 = Point2::new(r, pos.z);

        let distance2 = move |p: &FieldPoint| { (p.pos - p0).magnitude_squared() };
        self.iter()
            .map(|p| (distance2(p), p))
            .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
            .unwrap()
            .1
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::Vector2;
    use pretty_assertions::assert_eq;
    use float_eq::assert_float_eq;
    use crate::assert_vector2_eq;

    #[test]
    fn test_from_file() {
        use std::io::Write;
        use tempfile::NamedTempFile;

        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "1\n2\n3\n4\n5\n6\n7\n8").unwrap(); // 8 rows to skip
        writeln!(file, "1.2 -24.5 6 7.89").unwrap();       // so it's easy to test
        writeln!(file, "1.2 -14.5 6 7.89").unwrap();       // repeat the same values
        writeln!(file, "1.2  -4.5 6 7.89").unwrap();       // the actual rows

        let field = Field::from_file(file.path().to_str().unwrap());

        assert_eq!(field.len(), 2); // first one is skipped for being the end of the line

        let dir = Vector2::new(0.0, 1.0);
        for (i, fp) in field.iter().enumerate() {
            let z = -14.5 - (10*i) as f64;
            assert_float_eq!  (fp.r(),  1.2, ulps <= 2);
            assert_float_eq!  (fp.z(),    z, ulps <= 2);
            assert_float_eq!  (fp.mag, 7.89, ulps <= 2);
            assert_eq!        (fp.line, 6usize);
            assert_vector2_eq!(fp.dir,  dir, ulps <= 2);
        }
    }

    #[test]
    fn test_find_nearest() {
        let points = vec![
            FieldPoint::new(Point2::<f64>::new(0., 0.), 1., 0, Vector2::zeros()), // 0
            FieldPoint::new(Point2::<f64>::new(1., 1.), 1., 0, Vector2::zeros()), // 1
            FieldPoint::new(Point2::<f64>::new(2., 2.), 1., 0, Vector2::zeros()), // 2
            FieldPoint::new(Point2::<f64>::new(3., 3.), 1., 0, Vector2::zeros()), // 3
        ];

        let field = Field::new(points.clone());
        let p0 = Point3::new(0.1, 0.0, 0.1); // closest is 0
        let p1 = Point3::new(0.9, 0.0, 0.9); // closest is 1
        let p2 = Point3::new(1.9, 0.0, 2.1); // closest is 2
        let p3 = Point3::new(3.1, 0.0, 2.9); // closest is 3

        let c0 = field.find_nearest(&p0);
        let c1 = field.find_nearest(&p1);
        let c2 = field.find_nearest(&p2);
        let c3 = field.find_nearest(&p3);

        assert_float_eq!(c0.r(), points[0].r(), ulps <= 2);
        assert_float_eq!(c1.r(), points[1].r(), ulps <= 2);
        assert_float_eq!(c2.r(), points[2].r(), ulps <= 2);
        assert_float_eq!(c3.r(), points[3].r(), ulps <= 2);
    }

}

use nalgebra::{Point2, Point3, Vector2, Vector3};

#[derive(Clone, Debug)]
pub struct FieldPoint {
    pub pos : Point2<f64>,
    pub mag : f64,
    pub line: usize,
    pub dir : Vector2<f64>,
}

impl FieldPoint {
    pub fn new(pos: Point2<f64>, mag: f64, line: usize, dir: Vector2<f64>) -> Self {
        let dir = if dir.magnitude() > 0f64 { dir.normalize() } else {dir};
        FieldPoint{pos, mag, line, dir: dir}
    }

    pub fn r(&self) -> f64 { self.pos.x }
    pub fn z(&self) -> f64 { self.pos.y }

    pub fn from_csv_row(row: Vec<f64>, previous_pos: &mut Point2<f64>, current_line: &mut usize) -> Option<Self> {
        let (r, z, line, mag) = (row[0], row[1], row[2] as usize, row[3]);
        let current_pos = Point2::new(r, z);

        if line != *current_line {
            *previous_pos = current_pos;
            *current_line = line;
            return None;
        }

        let dir = (*previous_pos - current_pos).normalize();
        *previous_pos = current_pos;

        Some(Self::new(current_pos, mag, line, dir))
    }

    pub fn to_vec3(&self, p: &Point3<f64>) -> Vector3<f64> {
        let alpha = (p.y/p.x).atan();
        Vector3::new( self.dir.x * alpha.cos()
                    , self.dir.x * alpha.sin()
                    , self.dir.y)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use float_eq::assert_float_eq;
    use crate::{assert_point2_eq, assert_vector2_eq, assert_vector3_eq};

    #[test]
    fn test_field_point() {
        let pos =  Point2::new(1., 2.);
        let dir = Vector2::new(3., 4.);
        let fp  = FieldPoint::new(pos.clone(), 5., 6, dir.clone());

        assert_point2_eq! (fp.pos , pos, ulps<=2);
        assert_float_eq!  (fp.mag ,  5., ulps<=2);
        assert_eq!        (fp.line,   6);
        assert_vector2_eq!(fp.dir, dir.normalize(), ulps<=2);
    }

    #[test]
    fn test_process_row_same_line() {
        let row = vec![1.23, -4.56, 7., 8.9];
        let mut previous_pos = Point2::<f64>::origin();
        let mut current_line = 7usize;
        let dir = Vector2::new(-row[0], -row[1]).normalize();

        let fp = FieldPoint::from_csv_row(row.clone(), &mut previous_pos, &mut current_line).unwrap();

        // properties of output
        assert_float_eq!  (fp.r(),  1.23, ulps <= 2);
        assert_float_eq!  (fp.z(), -4.56, ulps <= 2);
        assert_float_eq!  (fp.mag,  8.90, ulps <= 2);
        assert_eq!        (fp.line, 7usize);
        assert_vector2_eq!(fp.dir,  dir, ulps <= 2);
        assert_float_eq!  (fp.dir.magnitude(),  1f64, ulps <= 2);

        // side effects
        let current_pos = Point2::new(row[0], row[1]);
        assert_eq!        (current_line, 7usize);
        assert_point2_eq!(previous_pos, current_pos, ulps <= 2);
    }

    #[test]
    fn test_process_row_different_line() {
        let row = vec![1.23, -4.56, 7., 8.9];
        let mut previous_pos = Point2::<f64>::origin();
        let mut current_line = 6usize;

        let fp = FieldPoint::from_csv_row(row.clone(), &mut previous_pos, &mut current_line);
        assert!(fp.is_none());

        // side effects
        let current_pos = Point2::new(row[0], row[1]);
        assert_eq!        (current_line, 7usize);
        assert_point2_eq!(previous_pos, current_pos, ulps <= 2);
    }

    #[test]
    fn test_to_vec3_z() {
        let dir = Vector2::new(0., 1.);
        let fp  = FieldPoint::new(Point2::origin(), 0., 0, dir);

        let pos      = Point3::new(1., 1., 1.);
        let new_dir  = fp.to_vec3(&pos);
        let expected = Vector3::z();
        assert_vector3_eq!(new_dir, expected, ulps <= 2);
    }

    #[test]
    fn test_to_vec3_r() {
        let dir = Vector2::new(1., 0.);
        let fp  = FieldPoint::new(Point2::origin(), 0., 0, dir);

        let pos      = Point3::new(1., 1., 1.);
        let new_dir  = fp.to_vec3(&pos);
        let expected = Vector3::new(1., 1., 0.).normalize();
        assert_vector3_eq!(new_dir, expected, ulps <= 2);
    }

    #[test]
    fn test_to_vec3_rz() {
        let dir = Vector2::new(2f64.sqrt(), 1.);
        let fp  = FieldPoint::new(Point2::origin(), 0., 0, dir);

        let pos      = Point3::new(1., 1., 1.);
        let new_dir  = fp.to_vec3(&pos);
        let expected = Vector3::new(1., 1., 1.).normalize();
        assert_vector3_eq!(new_dir, expected, ulps <= 2);
    }

}

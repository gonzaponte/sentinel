use nalgebra::{Point3, Vector3, Rotation3};

use crate::io::read_csv;
use crate::utils::interpolate_1d;
use crate::geometry::Geometry;
use crate::field::Field;
use crate::field_point::FieldPoint;
use crate::random;

#[derive(Clone, Debug)]
pub struct Tracker {
    field     : Field,
    geometry  : Geometry,
    drift     : (Vec<f64>, Vec<f64>),
    tstep     : f64
}

impl Tracker {
    pub fn new(field: Field, geometry: Geometry, tstep: f64) -> Self {
        let data    = read_csv("data/drift_velocity_gushchin.dat", ";", 1).unwrap();
        let drift_e = data.iter().map(|x| *x.first().unwrap()).collect::<Vec<f64>>(); // drift field
        let drift_v = data.iter().map(|x| *x. last().unwrap()).collect::<Vec<f64>>(); // drift velocity

        Self{ field
            , geometry
            , drift     : (drift_e, drift_v)
            , tstep
            }
    }

    pub fn propagate_from(&self, pos: Point3<f64>) -> Vec<Point3<f64>> {
        let mut trajectory : Vec<Point3<f64>> = Vec::with_capacity(1_000_000);
        trajectory.push(pos);

        while self.geometry.is_within(trajectory.last().unwrap()) {
            let current_pos = trajectory.last().unwrap();
            let direction   = self.compute_step(&current_pos);
            let next_pos = current_pos + direction;
            trajectory.push(next_pos);
        }
        trajectory
    }

    fn compute_step(&self, pos: &Point3<f64>) -> Vector3<f64> {
        let closest = self.field.find_nearest(pos);

        let dv          = self.drift_velocity(closest.mag);
        let step_length = dv * self.tstep;
        let field_dir   = closest.to_vec3(&pos);                           // direction set by e field
        let smeared_dir = self.randomize_direction(&closest, step_length); // diffusion smeared

        let rot  = Rotation3::rotation_between(&(-Vector3::z()), &field_dir).unwrap_or(Rotation3::identity());
        let step = rot * smeared_dir;

        step
    }

    fn randomize_direction(&self, fp: &FieldPoint, step_length: f64) -> Vector3<f64> {
        let dt = self.  transverse_diffusion(fp.mag);
        let dl = self.longitudinal_diffusion(fp.mag);
        let sigmat = (2f64 * dt * self.tstep * 1e-4).sqrt(); // 1e-4 = cm2/s to mm2/us
        let sigmal = (2f64 * dl * self.tstep * 1e-4).sqrt();

        let smearx = random::normal(0., sigmat);
        let smeary = random::normal(0., sigmat);
        let smearz = random::normal(0., sigmal);

        Vector3::new(smearx, smeary, step_length + smearz) // step_length positive to go upwards!!!
    }

    fn drift_velocity(&self, edrift: f64) -> f64 {
        interpolate_1d(&self.drift.0, &self.drift.1 , edrift).unwrap()
    }

    fn transverse_diffusion(&self, edrift: f64) -> f64 {
        // from NEST, parametrization of red data points in
        // https://arxiv.org/pdf/1609.04467
        37.368 * edrift.powf(0.093452) * (-8.1651e-5 * edrift).exp()
    }

    fn longitudinal_diffusion(&self, edrift: f64) -> f64 {
        // from NEST, parametrization of magenta data points in
        // https://arxiv.org/pdf/1911.11580
        66.350 * edrift.powf(-0.24855) + 36.85 * (-edrift / 35.661).exp()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_eq::assert_float_eq;

    fn default_geometry() -> Geometry { Geometry::new(1., 1., 0., 1.) }
    fn default_field   () -> Field    { Field::from_file("data/partial_efield.dat", 1., 1.) }
    fn default_tracker () -> Tracker  { Tracker ::new(default_field(), default_geometry(), 1.) }

    #[test]
    fn test_tracker_new() {
        let tracker = default_tracker();
        assert!(tracker.drift.0.len() > 1);
        assert!(tracker.drift.1.len() > 1);
    }

    #[test]
    fn test_drift_velocity() {
        let tracker = default_tracker();
        assert_float_eq!(tracker.drift_velocity( 100.), 1.4938051013386355, rel <= 1e-7);
        assert_float_eq!(tracker.drift_velocity(1000.), 2.2215991423542030, rel <= 1e-7);
        assert_float_eq!(tracker.drift_velocity(2000.), 2.5070676356146624, rel <= 1e-7);
    }
}

use serde::Deserialize;

use crate::sampler::{Sampler, FixedPositionSampler, EdgeVolumeSampler, SurfaceSampler, VolumeSampler};
use crate::geometry::Geometry;

use nalgebra::point;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum Generator {
    FixedPos(f64, f64, f64),
    CathodeCenter,
    Center,
    Volume,
    Surface,
    Edge(f64),
    FromFile(String),
}

impl Generator {
    pub fn sampler<G: Geometry>(&self, geo: &dyn Geometry) -> Box<dyn Sampler<G>>
    where
            VolumeSampler: Sampler<G>,
           SurfaceSampler: Sampler<G>,
        EdgeVolumeSampler: Sampler<G>,
    {
        match self.clone() {
            Generator::FixedPos(x,y,z)  => Box::new(FixedPositionSampler(point![x, y, z])),
            Generator::CathodeCenter    => Box::new(FixedPositionSampler(point![0., 0., geo.cathode_z() - 1.0])),
            Generator::Center           => Box::new(FixedPositionSampler(point![0., 0., geo.cathode_z() / 2.0])),
            Generator::Volume           => Box::new(       VolumeSampler{}),
            Generator::Surface          => Box::new(      SurfaceSampler{}),
            Generator::Edge(d)          => Box::new(   EdgeVolumeSampler(d)),
            Generator::FromFile(_p)     => unimplemented!(),
        }
    }
}

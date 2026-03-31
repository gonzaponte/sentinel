use hdf5_metno as hdf5;

#[derive(hdf5::H5Type, Clone, PartialEq, Debug)]
#[repr(C)]
pub struct IonizationHit {
    pub event   : u64,
    pub track_id: u16,
    pub x       : f32,
    pub y       : f32,
    pub z       : f32,
    pub e       : f32,
}

#[derive(hdf5::H5Type, Clone, PartialEq, Debug)]
#[repr(C)]
pub struct Endpoint {
    pub event: u32,
    pub x0   : f32,
    pub y0   : f32,
    pub z0   : f32,
    pub x1   : f32,
    pub y1   : f32,
    pub z1   : f32,
    pub t    : f32,
}

#[derive(hdf5::H5Type, Clone, PartialEq, Debug)]
#[repr(C)]
pub struct TrajectoryPoint {
    pub event: u32,
    pub x    : f32,
    pub y    : f32,
    pub z    : f32,
    pub t    : f32,
}

#[derive(hdf5::H5Type, Clone, PartialEq, Debug)]
#[repr(C)]
pub struct S1LightTablePoint {
    pub sensor_id: u16,
    pub z        : f32,
    pub x        : f32,
    pub y        : f32,
    pub nevt     : u32,
    pub prob     : f32,
}


#[derive(hdf5::H5Type, Clone, PartialEq, Debug)]
#[repr(C)]
pub struct S2LightTablePoint {
    pub sensor_id: u16,
    pub x        : f32,
    pub y        : f32,
    pub nevt     : u32,
    pub prob     : f32,
}

#[derive(hdf5::H5Type, Clone, PartialEq, Debug)]
#[repr(C)]
pub struct DriftMapPoint {
    pub r0   : f32,
    pub z0   : f32,
    pub x1   : f32,
    pub y1   : f32,
    pub n_dst: u32,
    pub n_src: u32,
    pub t_ave: f32,
    pub ok   : bool,
}

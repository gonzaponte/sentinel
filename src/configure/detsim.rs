use config::{Config, ConfigError, File};
use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Clone)]
pub struct Configure {
    pub rmin           : f64,
    pub form_factor    : f64,
    pub  neck_length   : f64,
    pub drift_length   : f64,
    pub field_file     : String,
    pub field_to_mm    : f64,
    pub field_to_Vpercm: f64,
    pub field_invert_z : bool,
    pub t_step         : f64,
    pub lifetime       : f64,
}

impl Configure {
    pub fn new(filename: &str) -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name(filename))
            .build()?;

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_deserialize()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use float_eq::assert_float_eq;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_new() {
        let c = Configure::new("conf/test.toml").unwrap();
        assert_float_eq!(c.rmin           ,  1.0, ulps<=2);
        assert_float_eq!(c.form_factor    ,  1.0, ulps<=2);
        assert_eq!      (c.field_file     , "data/homogeneous_field.dat");
        assert_float_eq!(c.field_to_mm    ,  1.0, ulps<=2);
        assert_float_eq!(c.field_to_Vpercm,  1e3, ulps<=2);
        assert!         (c.field_invert_z );
        assert_float_eq!(c. neck_length   ,  1.0, ulps<=2);
        assert_float_eq!(c.drift_length   , 10.0, ulps<=2);
        assert_float_eq!(c.t_step         , 1e-4, ulps<=2);
        assert_float_eq!(c.lifetime       , 123., ulps<=2);
    }

    #[test]
    fn test_new_err() {
        assert!(Configure::new("does_not_exist").is_err());
    }
}

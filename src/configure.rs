use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Configure {
    pub rmin       : f64,
    pub form_factor: f64,
    pub zmin       : f64,
    pub zmax       : f64,
    pub field_file : String,
    pub t_step     : f64,
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

    #[test]
    fn test_new() {
        let c = Configure::new("conf/test.toml").unwrap();
        assert_float_eq!(c.rmin       ,  1.0, ulps<=2);
        assert_float_eq!(c.form_factor,  1.0, ulps<=2);
        assert_float_eq!(c.zmin       ,  0.0, ulps<=2);
        assert_float_eq!(c.zmax       , 10.0, ulps<=2);
    }

    #[test]
    fn test_new_err() {
        assert!(Configure::new("does_not_exist").is_err());
    }
}

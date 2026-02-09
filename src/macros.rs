
#[cfg(test)]
#[macro_export]
macro_rules! assert_point2_eq {
    ($a:expr, $b:expr, $($rest:tt)+) => {{
        float_eq::assert_float_eq!($a.x, $b.x, $($rest)+);
        float_eq::assert_float_eq!($a.y, $b.y, $($rest)+);
    }};
}


#[cfg(test)]
#[macro_export]
macro_rules! assert_vector2_eq {
    ($a:expr, $b:expr, $($rest:tt)+) => {{
        $crate::assert_point2_eq!($a, $b, $($rest)+);
    }};
}


#[cfg(test)]
#[macro_export]
macro_rules! assert_point3_eq {
    ($a:expr, $b:expr, $($rest:tt)+) => {{
        float_eq::assert_float_eq!($a.x, $b.x, $($rest)+);
        float_eq::assert_float_eq!($a.y, $b.y, $($rest)+);
        float_eq::assert_float_eq!($a.z, $b.z, $($rest)+);
    }};
}


#[cfg(test)]
#[macro_export]
macro_rules! assert_vector3_eq {
    ($a:expr, $b:expr, $($rest:tt)+) => {{
        $crate::assert_point3_eq!($a, $b, $($rest)+);
    }};
}

#[macro_export]
macro_rules! invalid_input {
    ($message:expr) => {{
        Err(std::io::Error::new( std::io::ErrorKind::InvalidInput
                               , $message
                               )
           )
    }};
}

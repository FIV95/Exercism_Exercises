// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    seconds: u64,
}

impl Duration {
    pub fn new(s: u64) -> Self {
        Duration {seconds: s}
    }
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        return Duration::new(s)
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        todo!("convert a duration ({d:?}) to the number of years on this planet for that duration");
    }
}

macro_rules! define_planet {
    ($planet_name:ident, $orbital_ratio:expr) => {
        pub struct $planet_name;

        impl $planet_name {
            pub fn years_during(d: &Duration) -> f64 {
                let earth_seconds = 31_557_600;
                let ratio = ($orbital_ratio * earth_seconds as f64);
                (d.seconds as f64) / ratio
            }
        }
    }
}

define_planet!(Mercury, 0.2408467);
define_planet!(Venus, 0.61519726);
define_planet!(Earth, 1.0);
define_planet!(Mars, 1.8808158);
define_planet!(Jupiter, 11.862615);
define_planet!(Saturn, 29.447498);
define_planet!(Uranus, 84.016846);
define_planet!(Neptune, 164.79132);
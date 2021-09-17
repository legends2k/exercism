#[derive(Debug)]
pub struct Duration {
  secs: u64,
}

impl From<u64> for Duration {
  fn from(secs: u64) -> Self {
    Duration { secs }
  }
}

pub trait Planet {
  const SECONDS_IN_A_YEAR: f64 = 24.0 * 60.0 * 60.0 * 365.25;

  // https://users.rust-lang.org/t/best-practices-when-defining-a-default-implementation-for-a-traits-method/2033
  fn years_during(d: &Duration) -> f64 {
    d.secs as f64
      / (Self::orbital_period_in_earth_years() * Self::SECONDS_IN_A_YEAR)
  }

  /// Returns orbital period in Earth years
  fn orbital_period_in_earth_years() -> f64;
}

macro_rules! planet {
  ($name:ident, $period:literal) => {
    pub struct $name;
    impl Planet for $name {
      fn orbital_period_in_earth_years() -> f64 {
        $period
      }
    }
  };
}

planet!(Mercury, 0.2408467);
planet!(Venus, 0.61519726);
planet!(Earth, 1.0);
planet!(Mars, 1.8808158);
planet!(Jupiter, 11.862615);
planet!(Saturn, 29.447498);
planet!(Uranus, 84.016846);
planet!(Neptune, 164.79132);

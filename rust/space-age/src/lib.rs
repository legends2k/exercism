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

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
  fn orbital_period_in_earth_years() -> f64 {
    0.2408467
  }
}
impl Planet for Venus {
  fn orbital_period_in_earth_years() -> f64 {
    0.61519726
  }
}
impl Planet for Earth {
  fn orbital_period_in_earth_years() -> f64 {
    1.0
  }
}
impl Planet for Mars {
  fn orbital_period_in_earth_years() -> f64 {
    1.8808158
  }
}
impl Planet for Jupiter {
  fn orbital_period_in_earth_years() -> f64 {
    11.862615
  }
}
impl Planet for Saturn {
  fn orbital_period_in_earth_years() -> f64 {
    29.447498
  }
}
impl Planet for Uranus {
  fn orbital_period_in_earth_years() -> f64 {
    84.016846
  }
}
impl Planet for Neptune {
  fn orbital_period_in_earth_years() -> f64 {
    164.79132
  }
}

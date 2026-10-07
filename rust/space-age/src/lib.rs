const SECONDS_A_YEAR: f64 = 31557600.0;

#[derive(Debug)]
pub struct Duration(f64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self(s as f64)
    }
}

pub trait Planet {
    fn period() -> f64;

    fn years_during(d: &Duration) -> f64 {
        d.0 / SECONDS_A_YEAR / Self::period()
    }
}

macro_rules! planets {
    // for every "identifier => expression" seperated by ','
    ( $( $n:ident => $p:expr ),* ) => {
        $(
            pub struct $n;
            impl Planet for $n {
                fn period() -> f64 { $p }
            }
        )*
    };
}

planets! {
    Mercury => 0.2408467,
    Venus => 0.61519726,
    Earth => 1.0,
    Mars => 1.8808158,
    Jupiter => 11.862615,
    Saturn => 29.447498,
    Uranus => 84.016846,
    Neptune => 164.79132
}
use statrs::distribution::{ContinuousCDF, Normal};

struct EuropeanOption {
    option_type: &'static str,
    price: f64,
    strike: f64,
    interest_rate: f64,
    volatility: f64,
    time_to_maturity: i8,
    amount_underlying: i8,
}

impl EuropeanOption {
    fn value(&self) -> f64 {
        let t: f64 = self.time_to_maturity as f64;
        let amount_underlying: f64 = self.amount_underlying as f64;

        let normal_distribution = Normal::new(0.0, 1.0).unwrap();

        let d1 = ((self.price / self.strike).ln()
            + (self.interest_rate + self.volatility.powf(2.0) / 2.0))
            / (self.volatility * t.sqrt());

        let d2 = ((self.price / self.strike).ln()
            + (self.interest_rate - self.volatility.powf(2.0) / 2.0))
            / (self.volatility * t.sqrt());

        let value: f64 = if self.option_type == "call" {
            self.price * normal_distribution.cdf(d1)
                - self.strike * (-self.interest_rate * t).exp() * normal_distribution.cdf(d2)
        } else {
            -self.price * normal_distribution.cdf(-d1)
                + self.strike * (-self.interest_rate * t).exp() * normal_distribution.cdf(d2)
        };

        value * amount_underlying
    }
}

fn main() {
    let strike: f64 = 0.9;
    let sigma: f64 = 0.2;
    let r: f64 = 0.015;
    let price: f64 = 1.0;
    let time_to_maturity: i8 = 1;
    let amount_underlying: i8 = 1;

    let call_option = EuropeanOption {
        option_type: "call",
        price,
        strike,
        interest_rate: r,
        volatility: sigma,
        time_to_maturity,
        amount_underlying,
    };

    let c_0 = call_option.value();

    let put_option = EuropeanOption {
        option_type: "put",
        price,
        strike,
        interest_rate: r,
        volatility: sigma,
        time_to_maturity,
        amount_underlying,
    };

    let p_0 = put_option.value();

    assert_eq!(c_0, 0.14498531543284654);
    assert_eq!(p_0, 0.37221239391036487);

    println!("{c_0}");
    println!("{p_0}");
}

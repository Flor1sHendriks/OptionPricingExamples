
use statrs::distribution::{ContinuousCDF, Normal};
use std::env;
use std::path::Path;
use std::str::FromStr;

struct EuropeanOption {
    option_type: String,
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

fn test_european_option() {
    let strike: f64 = 0.9;
    let sigma: f64 = 0.2;
    let r: f64 = 0.015;
    let price: f64 = 1.0;
    let time_to_maturity: i8 = 1;
    let amount_underlying: i8 = 1;

    let call_option = EuropeanOption {
        option_type: String::from_str("call").unwrap(),

        price,
        strike,
        interest_rate: r,
        volatility: sigma,
        time_to_maturity,
        amount_underlying,
    };

    let c_0 = call_option.value();

    let put_option = EuropeanOption {
        option_type: String::from_str("put").unwrap(),
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

    println!("values are as expected");
}

fn main() {
    let current_dir = env::current_dir().unwrap();

    let csv_folder = Path::new(&current_dir).parent().unwrap();

    let csv_location = csv_folder.join("option_examples.csv");

    let mut reader = csv::Reader::from_path(&csv_location).unwrap();

    let mut list_of_options = Vec::new();

    for record in reader.records() {
        let record = record.unwrap();

        // serialize
        let option_type = String::from_str(&record[0]).unwrap();
        let price: f64 = record[1].parse::<f64>().unwrap();
        let strike = record[2].parse::<f64>().unwrap();
        let interest_rate = record[3].parse::<f64>().unwrap();
        let volatility = record[4].parse::<f64>().unwrap();
        let time_to_maturity = record[5].parse::<i8>().unwrap();
        let amount_underlying = record[6].parse::<i8>().unwrap();

        let call_option = EuropeanOption {
            option_type,
            price,
            strike,
            interest_rate,
            volatility,
            time_to_maturity,
            amount_underlying,
        };

        list_of_options.push(call_option);
    }

    // initialisation necessary otherwise c_0 is not available after the loop.
    let mut c_0=  1.0;
    for option in list_of_options {
        c_0 = option.value();
    }

    println!("{c_0}");

    test_european_option();
}

fn main() {

    let amount_underlying: i8 = 100;
    let strike: f64 = 1.1;
    let sigma: f64 = 0.2;
    let mu: f64 = 0.06;
    let r: f64 = 0.015;
    let s_0: i8 = 1;
    let t: i8 = 1;
    let c_0 = call_value(amount_underlying, sigma, strike, t, r, s_0);

    println!("{c_0}");
}

// https://hackernoon.com/lets-graph-simple-moving-averages-using-rust
// https://hackernoon.com/rusts-ownership-and-borrowing-enforce-memory-safety

fn call_value(amount_underlying: i8, sigma: f64, strike: f64, t: i8, r: f64, s_0: i8) -> f64 {

    let d_1 = fun_d1(sigma, k, t, r, s_0);
    let d_2 = fun_d2(sigma, k, t, r, s_0);
    let temp = norm_cdf(d1) * x - norm_cdf(d2) * k * exp(-r * t);

    return amount_underlying * temp
}

fn fun_d1() {

}

fn fun_d2() {
    
}
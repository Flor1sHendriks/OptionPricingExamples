from math import exp, log, sqrt
from statistics import NormalDist

# from https://www.quantstart.com/articles/European-Vanilla-Call-Put-Option-Pricing-with-Python/


class EuropeanOption:
    def __init__(
        self,
        option_type: str,
        price: float,
        strike: float,
        interest_rate: float,
        volatility: float,
        time_to_maturity: int,
        amount_underlying: int,
    ):
        """
        Args:
            option_type: type of the option. Must be call or put
            price: price at time 0.
            strike: strike price.
            interest_rate: interest rate.
            volatility: volatility (sigma).
            time_to_maturity: time to the end of the option. Sometimes this is expressed as T-t
            amount_underlying: number of options bought.
        """

        if option_type not in ["call", "put"]:
            raise ValueError("Option type not supported.")

        self.type = option_type
        self.s = price
        self.k = strike
        self.r = interest_rate
        self.v = volatility
        self.t = time_to_maturity
        self.amount_underlying = amount_underlying

    @property
    def d1(self) -> float:
        numerator = log(self.s / self.k) + (self.r + self.v**2 / 2) * self.t
        denominator = self.v * sqrt(self.t)
        return numerator / denominator

    @property
    def d2(self) -> float:
        numerator = log(self.s / self.k) + (self.r - self.v**2 / 2) * self.t
        denominator = self.v * sqrt(self.t)
        return numerator / denominator

    @property
    def value(self) -> float:

        norm = NormalDist(mu=0.0, sigma=1.0)

        if self.type == "call":
            value = self.s * norm.cdf(self.d1) - self.k * exp(
                -self.r * self.t
            ) * norm.cdf(self.d2)
        else:
            value = -self.s * norm.cdf(-self.d1) + self.k * exp(
                -self.r * self.t
            ) * norm.cdf(self.d2)

        return value * self.amount_underlying


if __name__ == "__main__":
    strike = 0.9
    sigma = 0.2
    mu = 0.06
    r = 0.015
    S0 = 1
    t = 1

    C0 = EuropeanOption(
        option_type="call",
        price=S0,
        strike=strike,
        interest_rate=r,
        volatility=sigma,
        time_to_maturity=t,
        amount_underlying=1,
    ).value

    P0 = EuropeanOption(
        option_type="put",
        price=S0,
        strike=strike,
        interest_rate=r,
        volatility=sigma,
        time_to_maturity=t,
        amount_underlying=1,
    ).value

    assert C0 == 0.14498531543284665
    assert P0 == 0.3722123939103649

    print(C0)
    print(P0)

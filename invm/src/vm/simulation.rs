use rand::Rng;

use crate::vm::{Sensor, VM};

impl VM {
    pub fn simulate(&mut self) {

        let balance = self.expect_sensor_value(&Sensor::Balance);
        let mut stockprice = self.expect_sensor_value(&Sensor::Stockprice);
        let mut reputation = self.expect_sensor_value(&Sensor::Reputation);
        let shares = self.expect_sensor_value(&Sensor::Shares);
        let owned = self.expect_sensor_value(&Sensor::Owned);

        let mut r = rand::rng();
        let rep_shift = r.random_range(-5..=5);
        reputation += rep_shift;

        stockprice += r.random_range(-5..=5);
        if stockprice < 0 {
            stockprice = 0;
        }

        let bias = (reputation - 50) * stockprice / 10;
        let factor = if bias < 0 {
            - (bias * bias)
        } else {
            bias * bias
        };

        let amount = factor * (shares - owned);
        let new = if amount + shares < 0 {
            owned
        } else {
            amount + shares
        };

        self.sensors.insert(Sensor::Shares, new);
        self.sensors.insert(Sensor::Stockprice, stockprice);
        self.sensors.insert(Sensor::Reputation, reputation);
        self.sensors.insert(Sensor::MarketValue, shares * stockprice);
        self.sensors.insert(Sensor::Equity, owned * stockprice);
        self.sensors.insert(Sensor::Balance, balance + 100);

    }
}

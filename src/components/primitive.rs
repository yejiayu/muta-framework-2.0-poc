pub type AccountID = String;

#[derive(Copy, Debug, Clone)]
pub struct Coin {
    value: u64,
}

impl Coin {
    pub fn new(value: u64) -> Self {
        Self { value }
    }

    pub fn get_value(&self) -> u64 {
        self.value
    }

    pub fn withdraw(&mut self, coin: Coin) {
        self.value = self.value - coin.get_value()
    }

    pub fn deposit(&mut self, coin: Coin) {
        self.value = self.value + coin.get_value()
    }
}

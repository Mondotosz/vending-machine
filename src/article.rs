pub struct Article {
    pub name: String,
    pub code: String,
    pub quantity: u32,
    pub price: f32, // potential issue. Using i32 and cents would prevent rounding errors
}

impl Article {
    pub fn new(code: String, name: String, quantity: u32, price: f32) -> Article {
        Article {
            code,
            name,
            quantity,
            price,
        }
    }

    pub fn remove(&mut self, amount: u32) {
        self.quantity -= amount;
    }

    pub fn add(&mut self, amount: u32) {
        self.quantity += amount;
    }
}

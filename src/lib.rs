#[cfg(test)]
mod tests;
pub mod article;

use crate::article::Article;

pub struct VendingMachine {
    articles: Vec<Article>,
    balance: f32, // Gains from sold articles
    change: f32,  // Currently inserted money
}

impl VendingMachine {
    pub fn new(articles: Vec<Article>) -> VendingMachine {
        VendingMachine {
            articles,
            balance: 0.0,
            change: 0.0,
        }
    }

    pub fn insert(&mut self, amount: f32) {
        self.change += amount;
    }

    // As stated in the task the return value is a string
    // A better idea would be to use an enum instead
    pub fn choose(&mut self, code: String) -> String {
        // Find the article
        let article = match self.articles.iter_mut().find(|a| a.code == code) {
            Some(article) => article,
            None => return String::from("Invalid selection!"),
        };

        // Check if enough money is inserted
        if self.change < article.price {
            return String::from("Not enough money!");
        };

        // Check if article is in stock
        if article.quantity == 0 {
            return format!("Item {}: out of stock!", article.name);
        };

        // Remove article from stock and update balance and change
        article.remove(1);
        self.balance += article.price;
        self.change -= article.price;

        return format!("Vending {}", article.name);
    }

    // Get the amount of money inserted which hasn't been used yet
    pub fn get_change(&self) -> f32 {
        self.change
    }

    // Get the amount of money the machine has earned
    pub fn get_balance(&self) -> f32 {
        self.balance
    }
}

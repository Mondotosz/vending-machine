use crate::article::Article;
pub mod article;

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
        let article = match self.articles.iter_mut().find(|a| a.code == code) {
            Some(article) => article,
            None => return String::from("Invalid selection!"),
        };

        if self.change < article.price {
            return String::from("Not enough money!");
        };

        if article.quantity == 0 {
            return format!("Item {}: out of stock!", article.name);
        };

        article.remove(1);
        self.balance += article.price;
        self.change -= article.price;

        return format!("Vending {}", article.name);
    }

    pub fn get_change(&self) -> f32 {
        self.change
    }

    pub fn get_balance(&self) -> f32 {
        self.balance
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_float_eq::*;

    fn setup() -> VendingMachine {
        VendingMachine::new(vec![
            Article::new(String::from("A01"), String::from("Smarlies"), 10, 1.60),
            Article::new(String::from("A02"), String::from("Carampar"), 5, 0.60),
            Article::new(String::from("A03"), String::from("Avril"), 2, 2.10),
            Article::new(String::from("A04"), String::from("KokoKola"), 1, 2.95),
        ])
    }

    #[test]
    fn first() {
        let mut vm = setup();

        vm.insert(3.40);

        assert_eq!(
            vm.choose(String::from("A01")),
            String::from("Vending Smarlies")
        );

        assert_float_relative_eq!(vm.get_change(), 1.80)
    }

    #[test]
    fn second() {
        let mut vm = setup();

        vm.insert(2.10);

        assert_eq!(
            vm.choose(String::from("A03")),
            String::from("Vending Avril")
        );

        assert_float_relative_eq!(vm.get_change(), 0.0);
        assert_float_relative_eq!(vm.get_balance(), 2.10);
    }

    #[test]
    fn third() {
        let mut vm = setup();

        assert_eq!(
            vm.choose(String::from("A01")),
            String::from("Not enough money!")
        );
    }

    #[test]
    fn fourth() {
        let mut vm = setup();

        vm.insert(1.00);

        assert_eq!(
            vm.choose(String::from("A01")),
            String::from("Not enough money!")
        );

        assert_float_relative_eq!(vm.get_change(), 1.00);

        assert_eq!(
            vm.choose(String::from("A02")),
            String::from("Vending Carampar")
        );

        assert_float_relative_eq!(vm.get_change(), 0.40);
    }

    #[test]
    fn fifth() {
        let mut vm = setup();

        vm.insert(1.00);

        assert_eq!(
            vm.choose(String::from("A05")),
            String::from("Invalid selection!")
        );
    }

    #[test]
    fn sixth() {
        let mut vm = setup();

        vm.insert(6.00);

        assert_eq!(
            vm.choose(String::from("A04")),
            String::from("Vending KokoKola")
        );

        assert_eq!(
            vm.choose(String::from("A04")),
            String::from("Item KokoKola: out of stock!")
        );

        assert_float_relative_eq!(vm.get_change(), 3.05);
    }

    #[test]
    fn seventh() {
        let mut vm = setup();

        vm.insert(6.00);

        assert_eq!(
            vm.choose(String::from("A04")),
            String::from("Vending KokoKola")
        );

        vm.insert(6.00);

        assert_eq!(
            vm.choose(String::from("A04")),
            String::from("Item KokoKola: out of stock!")
        );

        assert_eq!(
            vm.choose(String::from("A01")),
            String::from("Vending Smarlies")
        );

        assert_eq!(
            vm.choose(String::from("A02")),
            String::from("Vending Carampar")
        );

        assert_eq!(
            vm.choose(String::from("A02")),
            String::from("Vending Carampar")
        );

        assert_float_relative_eq!(vm.get_change(), 6.25);

        assert_float_relative_eq!(vm.get_balance(), 5.75);
    }
}

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

use super::*;
use assert_float_eq::*;
use time::macros::datetime;

fn setup() -> VendingMachine {
    VendingMachine::new(vec![
        Article::new(String::from("A01"), String::from("Smarlies"), 10, 1.60),
        Article::new(String::from("A02"), String::from("Carampar"), 5, 0.60),
        Article::new(String::from("A03"), String::from("Avril"), 2, 2.10),
        Article::new(String::from("A04"), String::from("KokoKola"), 1, 2.95),
    ])
}

#[test]
fn it_sells() {
    let mut vm = setup();

    vm.insert(3.40);

    assert_eq!(
        vm.choose(String::from("A01")),
        String::from("Vending Smarlies")
    );

    assert_float_relative_eq!(vm.get_change(), 1.80)
}

#[test]
fn it_transfers_change_to_balance_on_sell() {
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
fn it_checks_the_amount_of_change() {
    let mut vm = setup();

    assert_eq!(
        vm.choose(String::from("A01")),
        String::from("Not enough money!")
    );
}

#[test]
fn it_sells_only_when_enough_change_is_present() {
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
fn it_handles_invalid_codes() {
    let mut vm = setup();

    vm.insert(1.00);

    assert_eq!(
        vm.choose(String::from("A05")),
        String::from("Invalid selection!")
    );
}

#[test]
fn it_manages_stock() {
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
fn it_works() {
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

// Extension

fn ext_setup() -> VendingMachine {
    VendingMachine::new(vec![
        Article::new(String::from("A01"), String::from("Smarlies"), 100, 1.60),
        Article::new(String::from("A02"), String::from("Carampar"), 50, 0.60),
        Article::new(String::from("A03"), String::from("Avril"), 20, 2.10),
        Article::new(String::from("A04"), String::from("KokoKola"), 10, 2.95),
    ])
}

#[test]
fn it_tracks_timestamps() {
    let mut vm = ext_setup();

    vm.insert(1000.00);

    vm.set_time(datetime!(2020-01-01 20:30:00).assume_utc());
    vm.choose(String::from("A01"));

    vm.set_time(datetime!(2020-03-01 23:30:00).assume_utc());
    vm.choose(String::from("A01"));

    vm.set_time(datetime!(2020-03-04 09:22:00).assume_utc());
    vm.choose(String::from("A01"));

    vm.set_time(datetime!(2020-04-01 23:00:00).assume_utc());
    vm.choose(String::from("A01"));

    vm.set_time(datetime!(2020-04-01 23:59:59).assume_utc());
    vm.choose(String::from("A01"));

    vm.set_time(datetime!(2020-04-04 09:12:00).assume_utc());
    vm.choose(String::from("A01"));

    // Get the timestamps
    let mut timestamps = vm.get_timestamps();

    // Sort by amount
    timestamps.sort_by(|left, right| right.amount.partial_cmp(&left.amount).unwrap());

    // Get the top 3
    let top_three = timestamps[0..3].to_vec();

    // Assert that the top three is correct

    // Hour 23 generated a revenue of 4.80
    assert_eq!(top_three[0].hour, 23);
    assert_float_relative_eq!(top_three[0].amount, 4.80);

    // Hour 9 generated a revenue of 3.20
    assert_eq!(top_three[1].hour, 9);
    assert_float_relative_eq!(top_three[1].amount, 3.20);

    // Hour 20 generated a revenue of 1.60
    assert_eq!(top_three[2].hour, 20);
    assert_float_relative_eq!(top_three[2].amount, 1.60);
}

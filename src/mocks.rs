use crate::data::Account;

pub fn make_accounts(n: u8) -> Vec<Account> {
    let mut accounts = vec![];
    for i in 0..n {
        accounts.push(Account::new(format!("Account #{}", i)))
    }
    accounts
}

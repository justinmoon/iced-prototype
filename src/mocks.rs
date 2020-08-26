use crate::data::Account;

pub fn make_accounts(n: u8) -> Vec<Account> {
    let mut accounts = vec![];
    for i in 0..n {
        accounts.push(
            Account::new(
                format!("Account #{}", i), 
                // This is a root xprv, should add derivation information later?? 
                "wpkh(tprv8ZgxMBicQKsPfQ1yv9g3AVtRQeK595CXXqt8scUTZEc8yF4ZAqZcB5RUwfD5pjnuRXh7pvynes5DNA8G9fCMY3HbqDxM8fv5zHeGgc2VRSY/0/*)".to_string()
            )
        )
    }
    accounts
}

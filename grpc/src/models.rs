use crate::schema::accounts;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

#[derive(Queryable, Clone)]
pub struct Account {
    pub id: i32,
    pub name: String,
    pub balance: i32,
    pub deleted: bool,
}

#[derive(Insertable)]
#[diesel(table_name = accounts)]
pub struct NewAccount<'a> {
    pub id: &'a i32,
    pub name: &'a str,
}

pub fn get_account_by_id(conn: &mut SqliteConnection, id: &i32) -> Account {
    let accounts = accounts::table
        .filter(accounts::id.eq(id))
        .limit(1)
        .load::<Account>(conn)
        .expect("Error loading account");

    accounts.first().unwrap().clone()
}

pub fn create_account(conn: &mut SqliteConnection, id: &i32, name: &str) -> Account {
    let new_account = NewAccount { id, name };

    diesel::insert_into(accounts::table)
        .values(&new_account)
        .execute(conn)
        .expect("Error saving new post");

    get_account_by_id(conn, id)
}

pub fn get_account_balance(conn: &mut SqliteConnection, id: &i32) -> i32 {
    let account = get_account_by_id(conn, id);
    account.balance
}

pub fn update_account_balance(conn: &mut SqliteConnection, id: &i32, balance: &i32) -> Account {
    let current_balance = get_account_balance(conn, id);
    let new_balance = current_balance + balance;

    diesel::update(accounts::table.filter(accounts::id.eq(id)))
        .set(accounts::balance.eq(new_balance))
        .execute(conn)
        .expect("Error updating account");

    get_account_by_id(conn, id)
}

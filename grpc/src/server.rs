use tonic::{transport::Server, Request, Response, Status};
mod bank;
use bank::bank_server::{Bank, BankServer};
use bank::{AccountRequest, AccountResponse};
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;
use std::env;

mod models;
mod schema;
use crate::bank::DepositRequest;
use crate::models::{get_account_balance, update_account_balance};
use models::create_account;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[derive(Default)]
pub struct BankImpl {}

#[tonic::async_trait]
impl Bank for BankImpl {
    async fn add_account(
        &self,
        request: Request<AccountRequest>,
    ) -> Result<Response<AccountResponse>, Status> {
        let account_request = request.into_inner();
        let mut conn = establish_connection();
        let account = create_account(&mut conn, &account_request.id, &account_request.name);

        let reply = bank::AccountResponse {
            id: account.id,
            name: account.name,
            balance: account.balance,
        };
        Ok(Response::new(reply))
    }

    async fn get_balance(
        &self,
        request: Request<AccountRequest>,
    ) -> Result<Response<AccountResponse>, Status> {
        let account_request = request.into_inner();
        let mut conn = establish_connection();

        let account_balance = get_account_balance(&mut conn, &account_request.id);

        let reply = bank::AccountResponse {
            id: account_request.id,
            name: account_request.name,
            balance: account_balance,
        };
        Ok(Response::new(reply))
    }

    async fn deposit(
        &self,
        request: Request<DepositRequest>,
    ) -> Result<Response<AccountResponse>, Status> {
        let deposit_request = request.into_inner();
        let mut conn = establish_connection();

        let new_balance_account =
            update_account_balance(&mut conn, &deposit_request.id, &deposit_request.amount);
        println!("new balance: {:?}", new_balance_account.balance);

        let reply = bank::AccountResponse {
            id: new_balance_account.id,
            name: new_balance_account.name,
            balance: new_balance_account.balance,
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // defining address for our service
    let addr = "[::1]:50051".parse().unwrap();
    // creating a service
    let bank = BankImpl::default();

    println!("bank server listening on {}", addr);
    // adding our service to our server.
    Server::builder()
        .add_service(BankServer::new(bank))
        .serve(addr)
        .await?;

    Ok(())
}

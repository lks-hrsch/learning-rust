mod bank;
use crate::bank::DepositRequest;
use bank::bank_client::BankClient;
use bank::AccountRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // creating a channel ie connection to server
    let channel = tonic::transport::Channel::from_static("http://[::1]:50051")
        .connect()
        .await?;
    // creating gRPC client from channel
    let mut client = BankClient::new(channel);

    /*
    // creating a new Request
    let request = tonic::Request::new(AccountRequest {
        id: 2,
        name: String::from("isabelle"),
    });
    // sending request and waiting for response
    let response = client.add_account(request).await?.into_inner();
    println!("{:#?}", response);
     */

    // creating a new Request
    let request = tonic::Request::new(DepositRequest { id: 2, amount: 100 });
    let response = client.deposit(request).await?.into_inner();
    println!("{:#?}", response);

    // creating a new Request
    let request = tonic::Request::new(AccountRequest {
        id: 2,
        name: String::from("isabelle"),
    });
    let response = client.get_balance(request).await?.into_inner();
    println!("{:#?}", response);
    Ok(())
}

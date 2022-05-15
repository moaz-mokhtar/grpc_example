use members::membership_client::MembershipClient;
use members::AddMemberRequest;

pub mod members {
    tonic::include_proto!("members");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = MembershipClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(
        AddMemberRequest {
            name: "Ahmed".to_owned(),
        }
    );

    let response = client.add_member(request).await?;

    println!("Membership RESPONSE = {:?}", response);

    Ok(())
}
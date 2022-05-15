use tonic::{transport::Server, Request, Response, Status};

use payments::bitcoin_server::{Bitcoin, BitcoinServer};
use payments::{BtcPaymentRequest, BtcPaymentResponse};

use members::membership_server::{Membership, MembershipServer};
use members::{AddMemberRequest, AddMemberResponse};

pub mod payments {
    tonic::include_proto!("payments");
}

pub mod members {
    tonic::include_proto!("members");
}

#[derive(Debug, Default)]
pub struct BitcoinService {}

#[tonic::async_trait]
impl Bitcoin for BitcoinService {
    async fn send_payment(
        &self,
        request: Request<BtcPaymentRequest>,
    ) -> Result<Response<BtcPaymentResponse>, Status> {
        println!("Got a request: {:?}", request);

        let req = request.into_inner();

        let reply = BtcPaymentResponse {
            successful: true,
            message: format!("Sent {}BTC to {}.", req.amount, req.to_addr).into(),
        };

        Ok(Response::new(reply))
    }
}

#[derive(Debug, Default)]
pub struct MembershipService {}

#[tonic::async_trait]
impl Membership for MembershipService {
    async fn add_member(
        &self,
        request: Request<AddMemberRequest>,
    ) -> Result<Response<AddMemberResponse>, Status> {
        println!("Got a request [add_membership]: {:?}", request);

        let req = request.into_inner();

        let reply = AddMemberResponse {
            successful: true,
            message: format!("Member {} added.", req.name).into(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let btc_service = BitcoinService::default();
    let membership_service = MembershipService::default();

    Server::builder()
        .add_service(BitcoinServer::new(btc_service))
        .add_service(MembershipServer::new(membership_service))
        .serve(addr)
        .await?;

    Ok(())
}

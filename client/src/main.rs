use clap::Parser;
use tonic::{metadata::MetadataValue, transport::Channel, Request};

use pb::{TimeRequest, time_service_client::TimeServiceClient};

pub mod pb {
    tonic::include_proto!("time_service");
}

#[derive(Parser, Debug)]
#[command(name = "time-service-client")]
#[command(bin_name = "time-service-client")]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Args {
    #[clap(long, short, action)]
    utc: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let args = Args::parse();

    let channel = Channel::from_static("http://[::1]:50051").connect().await?;

    let token: MetadataValue<_> = "Bearer deadbeef".parse()?;
    let mut client = TimeServiceClient::with_interceptor(channel, move |mut req: Request<()>| {
        req.metadata_mut().insert("authorization", token.clone());
        Ok(req)
    });

    let request = tonic::Request::new(TimeRequest { utc: args.utc });
    tracing::info!(message = "send req");

    let response = client.get_current_time(request).await?;

    tracing::info!(message = "recv res", response = %response.get_ref().value);

    Ok(())
}
use tonic::{metadata::MetadataValue, transport::Server, Request, Response, Status};
use pb::{TimeRequest, TimeResponse, time_service_server::{TimeService, TimeServiceServer}};
use chrono::{Local, Utc};

pub mod pb {
    tonic::include_proto!("time_service");
    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("time_service_descriptor");
}

type TimeServerResult<T> = Result<Response<T>, Status>;
type InterceptResult<T> = Result<Request<T>, Status>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let reflectsvc = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(pb::FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    let addr = "[::1]:50051".parse().unwrap();
    let server = LocalTimeServer::default();
    let timesvc = TimeServiceServer::with_interceptor(server, intercept);

    tracing::info!(message = "starting service on", %addr);

    Server::builder()
        .trace_fn(|_| tracing::info_span!("time_service_server"))
        .add_service(reflectsvc)
        .add_service(timesvc)
        .serve(addr)
        .await?;

    Ok(())
}

fn intercept(request: Request<()>) -> InterceptResult<()> {
    let token: MetadataValue<_> = "Bearer deadbeef".parse().unwrap();

    match request.metadata().get("authorization") {
        Some(t) if token == t => Ok(request),
        _ => Err(Status::unauthenticated("Invalid token")),
    }
}

#[derive(Debug, Default)]
pub struct LocalTimeServer {}

#[tonic::async_trait]
impl TimeService for LocalTimeServer {
    #[tracing::instrument]
    async fn get_current_time(&self, request: Request<TimeRequest>) -> TimeServerResult<TimeResponse> {
        let r = request.into_inner();
        tracing::trace!("received request: {:?}", r);

        let reply = match r.utc {
            true => {
                TimeResponse { value: Utc::now().to_string() } 
            },

            false => {
                TimeResponse { value: Local::now().to_string() }
            }
        };
        
        Ok(Response::new(reply))
    }
}

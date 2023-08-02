use aws_lambda_events::apigw::{ApiGatewayV2httpRequest, ApiGatewayV2httpResponse};
use lambda_runtime::{Error, LambdaEvent, run, service_fn};
use tracing::info;

async fn function_handler(request: LambdaEvent<ApiGatewayV2httpRequest>) -> Result<ApiGatewayV2httpResponse, Error> {
    info!("{:?}", request.payload);

    let response = ApiGatewayV2httpResponse {
        status_code: 200,
        ..ApiGatewayV2httpResponse::default()
    };

    Ok(response)
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    run(service_fn(function_handler)).await.unwrap();
}

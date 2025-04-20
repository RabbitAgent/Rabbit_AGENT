use actix_web::{web, App, HttpServer, HttpResponse};
use tonic::transport::Server as GrpcServer;
use serde_json::json;

mod blockchain;
mod tee;
mod auth;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Start gRPC server
    let grpc_addr = "[::1]:50051".parse().unwrap();
    let grpc_router = GrpcServer::builder()
        .add_service(tee::TrustedExecutionServiceServer::new(tee::TeeServiceImpl::new()))
        .serve(grpc_addr);

    tokio::spawn(grpc_router);

    // Start HTTP server
    HttpServer::new(|| {
        App::new()
            .wrap(auth::JwtMiddleware)
            .service(
                web::resource("/inference")
                    .route(web::post().to(handle_inference))
            )
            .service(
                web::resource("/model/{model_hash}")
                    .route(web::get().to(get_model_status))
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

async fn handle_inference(
    payload: web::Json<InferenceRequest>,
    blockchain: web::Data<blockchain::NodeClient>,
) -> HttpResponse {
    let verification = blockchain.verify_request_signature(
        &payload.signed_payload,
        &payload.signature
    ).await;

    if !verification.valid {
        return HttpResponse::Unauthorized().json(json!({"error": "Invalid signature"}));
    }

    let tee_result = tee::execute_in_tee(&payload.model_hash, &payload.input_data)
        .await
        .map_err(|e| HttpResponse::InternalServerError().json(json!({"error": e.to_string()})));

    match tee_result {
        Ok(output) => HttpResponse::Ok().json(output),
        Err(e) => e,
    }
}

// api-gateway/src/blockchain/mod.rs
use web3::types::H160;
use ethsign::{SecretKey, Signature};

pub struct NodeClient {
    web3: web3::Web3<web3::transports::Http>,
}

impl NodeClient {
    pub async fn verify_request_signature(
        &self,
        message: &[u8],
        signature: &Signature,
    ) -> bool {
        let recovery = signature
            .recover(message)
            .expect("Signature recovery failed");
        
        let signer_address = recovery.address;
        let contract_address = self.get_authorized_address().await;
        
        signer_address == contract_address
    }
}

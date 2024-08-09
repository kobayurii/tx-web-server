#[derive(
    borsh::BorshDeserialize,
    serde::Serialize,
    serde::Deserialize,
    Debug,
    Clone,
)]
pub struct TransactionDetails {
    pub receipts: Vec<near_indexer_primitives::views::ReceiptView>,
    pub receipts_outcome: Vec<near_indexer_primitives::views::ExecutionOutcomeWithIdView>,
    pub status: near_indexer_primitives::views::FinalExecutionStatus,
    pub transaction: near_indexer_primitives::views::SignedTransactionView,
    pub transaction_outcome: near_indexer_primitives::views::ExecutionOutcomeWithIdView,
}

#[derive(borsh::BorshDeserialize, serde::Serialize, Debug, Clone)]
pub struct TransactionDetailsV0201 {
    pub receipts: Vec<near_indexer_primitives_0_20_1::views::ReceiptView>,
    pub receipts_outcome: Vec<near_indexer_primitives_0_20_1::views::ExecutionOutcomeWithIdView>,
    pub status: near_indexer_primitives_0_20_1::views::FinalExecutionStatus,
    pub transaction: near_indexer_primitives_0_20_1::views::SignedTransactionView,
    pub transaction_outcome: near_indexer_primitives_0_20_1::views::ExecutionOutcomeWithIdView,
}

#[derive(borsh::BorshDeserialize, serde::Serialize, Debug, Clone)]
pub struct TransactionDetailsV0212 {
    pub receipts: Vec<near_indexer_primitives_0_21_2::views::ReceiptView>,
    pub receipts_outcome: Vec<near_indexer_primitives_0_21_2::views::ExecutionOutcomeWithIdView>,
    pub status: near_indexer_primitives_0_21_2::views::FinalExecutionStatus,
    pub transaction: near_indexer_primitives_0_21_2::views::SignedTransactionView,
    pub transaction_outcome: near_indexer_primitives_0_21_2::views::ExecutionOutcomeWithIdView,
}

#[derive(borsh::BorshDeserialize, serde::Serialize, Debug, Clone)]
pub struct TransactionDetailsV0220 {
    pub receipts: Vec<near_indexer_primitives_0_22_0::views::ReceiptView>,
    pub receipts_outcome: Vec<near_indexer_primitives_0_22_0::views::ExecutionOutcomeWithIdView>,
    pub status: near_indexer_primitives_0_22_0::views::FinalExecutionStatus,
    pub transaction: near_indexer_primitives_0_22_0::views::SignedTransactionView,
    pub transaction_outcome: near_indexer_primitives_0_22_0::views::ExecutionOutcomeWithIdView,
}

#[derive(borsh::BorshDeserialize, serde::Serialize, Debug, Clone)]
pub struct TransactionDetailsV0230 {
    pub receipts: Vec<near_indexer_primitives_0_23_0::views::ReceiptView>,
    pub receipts_outcome: Vec<near_indexer_primitives_0_23_0::views::ExecutionOutcomeWithIdView>,
    pub status: near_indexer_primitives_0_23_0::views::FinalExecutionStatus,
    pub transaction: near_indexer_primitives_0_23_0::views::SignedTransactionView,
    pub transaction_outcome: near_indexer_primitives_0_23_0::views::ExecutionOutcomeWithIdView,
}

// Deserialize old versions of the TransactionDetails
// This is needed to handle the backward incompatible changes in the TransactionDetails
enum TransactionDetailsOldVersion {
    V0201(TransactionDetailsV0201),
    V0212(TransactionDetailsV0212),
    V0220(TransactionDetailsV0220),
    V0230(TransactionDetailsV0230),
    V0240(TransactionDetails),
}

impl TransactionDetailsOldVersion {
    fn borsh_deserialize(data: &[u8]) -> anyhow::Result<Self> {
        match borsh::from_slice::<TransactionDetailsV0201>(data) {
            Ok(tx_details) => Ok(TransactionDetailsOldVersion::V0201(tx_details)),
            Err(err) => {
                println!("Failed to deserialize TransactionDetailsV0201 {}", err);
                match borsh::from_slice::<TransactionDetailsV0212>(data) {
                    Ok(tx_details) => Ok(TransactionDetailsOldVersion::V0212(tx_details)),
                    Err(err) => {
                        println!("Failed to deserialize TransactionDetailsV0212 {}", err);
                        match borsh::from_slice::<TransactionDetailsV0220>(data) {
                            Ok(tx_details) => Ok(TransactionDetailsOldVersion::V0220(tx_details)),
                            Err(err) => {
                                println!("Failed to deserialize TransactionDetailsV0220 {}", err);
                                match borsh::from_slice::<TransactionDetailsV0230>(data) {
                                    Ok(tx_details) => Ok(TransactionDetailsOldVersion::V0230(tx_details)),
                                    Err(err) => {
                                        println!("Failed to deserialize TransactionDetailsV0230 {}", err);
                                        match borsh::from_slice::<TransactionDetails>(data) {
                                            Ok(tx_details) => Ok(TransactionDetailsOldVersion::V0240(tx_details)),
                                            Err(err) => {
                                                println!("Failed to deserialize TransactionDetails {}", err);
                                                anyhow::bail!("Failed to deserialize TransactionDetails {}", err)
                                            }
                                        }
                                    },
                                }
                            },
                        }
                    },
                }
            }
        }
    }

    fn to_json(&self) -> anyhow::Result<serde_json::Value> {
        Ok(match self {
            TransactionDetailsOldVersion::V0201(tx_details) => serde_json::to_value(tx_details).map_err(|err| {
                println!("Failed to serialize TransactionDetailsV0201 {}", err);
                anyhow::anyhow!(err)})?,
            TransactionDetailsOldVersion::V0212(tx_details) => serde_json::to_value(tx_details).map_err(|err| {
                println!("Failed to serialize TransactionDetailsV0212 {}", err);
                anyhow::anyhow!(err)})?,
            TransactionDetailsOldVersion::V0220(tx_details) => serde_json::to_value(tx_details).map_err(|err| {
                println!("Failed to serialize TransactionDetailsV0220 {}", err);
                anyhow::anyhow!(err)})?,
            TransactionDetailsOldVersion::V0230(tx_details) => serde_json::to_value(tx_details).map_err(|err| {
                println!("Failed to serialize TransactionDetailsV0230 {}", err);
                anyhow::anyhow!(err)})?,
            TransactionDetailsOldVersion::V0240(tx_details) => serde_json::to_value(tx_details).map_err(|err| {
                println!("Failed to serialize TransactionDetails {}", err);
                anyhow::anyhow!(err)})?,
        })
    }
}

pub async fn fetch_gcs_tx(client: std::sync::Arc<google_cloud_storage::client::Client>, key: &str) -> anyhow::Result<Vec<u8>> {
    let data = client
        .download_object(
            &google_cloud_storage::http::objects::get::GetObjectRequest {
                bucket: "readrpc-mainnet-tx-details".to_string(),
                object: key.to_string(),
                ..Default::default()
            },
            &google_cloud_storage::http::objects::download::Range::default(),
        )
        .await?;
    Ok(data)
}

async fn get_tx_json(client: actix_web::web::Data<std::sync::Arc<google_cloud_storage::client::Client>>, file_name: actix_web::web::Path<String>) -> impl actix_web::Responder {

    match fetch_gcs_tx(client.get_ref().clone(), &file_name).await {
        Ok(content) => {
            match TransactionDetailsOldVersion::borsh_deserialize(&content) {
                Ok(tx) => {
                    match tx.to_json() {
                        Ok(tx) => actix_web::HttpResponse::Ok().json(tx),
                        Err(_) => actix_web::HttpResponse::InternalServerError().body("Failed to deserialize TransactionDetails"),
                    }
                }
                Err(_) => actix_web::HttpResponse::InternalServerError().body("Failed to deserialize TransactionDetails"),
            }
        }
        Err(_) => actix_web::HttpResponse::NotFound().body("File not found"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let gcs_config = google_cloud_storage::client::ClientConfig::default().with_auth().await.unwrap();
    let client = std::sync::Arc::new(google_cloud_storage::client::Client::new(gcs_config));

    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .app_data(actix_web::web::Data::new(client.clone())) // Share the client with each request
            .route("/get-tx/{file_name}", actix_web::web::get().to(get_tx_json))
    })
        .bind(("127.0.0.1", 9099))?
        .run()
        .await
}
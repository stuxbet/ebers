use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Represents a single data point from the CSV
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataPoint {
    pub index: u32,
    pub timestamp: f64,
    pub value: f64,
}

/// Metadata about the data collection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetMetadata {
    pub port: String,
    pub baud_rate: u32,
    pub collection_duration_ms: u64,
}

/// Request payload sent to the prediction API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionRequest {
    pub dataset_id: String,
    pub timestamp: String,
    pub row_count: usize,
    pub data: Vec<DataPoint>,
    pub metadata: DatasetMetadata,
}

/// Metadata returned in the API response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseMetadata {
    pub model_version: Option<String>,
    pub processing_time_ms: Option<u64>,
}

/// Successful response from the prediction API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionResponse {
    pub success: bool,
    pub dataset_id: String,
    pub probability: f64,
    pub confidence: Option<f64>,
    pub processed_at: String,
    pub metadata: Option<ResponseMetadata>,
}

/// Error details from the API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiErrorDetails {
    pub code: String,
    pub message: String,
    pub details: Option<String>,
}

/// Error response from the prediction API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionErrorResponse {
    pub success: bool,
    pub error: ApiErrorDetails,
}

/// Result type for prediction operations
pub type PredictionResult = Result<PredictionResponse, String>;

/// API client for making prediction requests
#[derive(Clone)]
pub struct PredictionApiClient {
    endpoint: String,
    client: reqwest::Client,
    max_retries: u32,
    timeout: Duration,
}

impl PredictionApiClient {
    /// Create a new API client with the given endpoint
    pub fn new(endpoint: String) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            endpoint,
            client,
            max_retries: 3,
            timeout: Duration::from_secs(30),
        }
    }

    /// Make a prediction request with retry logic
    pub async fn predict(&self, request: PredictionRequest) -> PredictionResult {
        let mut last_error = String::new();

        for attempt in 1..=self.max_retries {
            println!(
                "[api_client] Attempt {}/{} to call prediction API",
                attempt, self.max_retries
            );

            match self.make_request(&request).await {
                Ok(response) => {
                    println!(
                        "[api_client] Prediction successful: probability={}",
                        response.probability
                    );
                    return Ok(response);
                }
                Err(err) => {
                    last_error = err.clone();
                    println!("[api_client] Attempt {} failed: {}", attempt, err);

                    if attempt < self.max_retries {
                        // Exponential backoff: 1s, 2s, 4s
                        let backoff_ms = 1000 * (1 << (attempt - 1));
                        println!("[api_client] Retrying in {}ms...", backoff_ms);
                        tokio::time::sleep(Duration::from_millis(backoff_ms)).await;
                    }
                }
            }
        }

        Err(format!(
            "Failed after {} attempts. Last error: {}",
            self.max_retries, last_error
        ))
    }

    /// Make a single HTTP request to the API
    async fn make_request(&self, request: &PredictionRequest) -> PredictionResult {
        // Make the POST request
        let response = self
            .client
            .post(&self.endpoint)
            .json(request)
            .send()
            .await
            .map_err(|e| {
                if e.is_timeout() {
                    "Request timeout - API took too long to respond".to_string()
                } else if e.is_connect() {
                    "Cannot connect to API - check if the service is running".to_string()
                } else {
                    format!("Network error: {}", e)
                }
            })?;

        let status = response.status();

        // Handle successful response (200-299)
        if status.is_success() {
            let prediction = response
                .json::<PredictionResponse>()
                .await
                .map_err(|e| format!("Failed to parse API response: {}", e))?;

            return Ok(prediction);
        }

        // Handle error responses (4xx, 5xx)
        if let Ok(error_response) = response.json::<PredictionErrorResponse>().await {
            return Err(format!(
                "API error [{}]: {}",
                error_response.error.code, error_response.error.message
            ));
        }

        // Fallback for unexpected responses
        Err(format!("API returned error status: {}", status))
    }
}

/// Parse CSV buffer into structured data points
pub fn parse_csv_data(csv_buffer: &str) -> Result<Vec<DataPoint>, String> {
    let mut data_points = Vec::new();

    for (line_num, line) in csv_buffer.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() != 3 {
            return Err(format!(
                "Invalid CSV format at line {}: expected 3 columns, got {}",
                line_num + 1,
                parts.len()
            ));
        }

        let index = parts[0]
            .trim()
            .parse::<u32>()
            .map_err(|e| format!("Invalid index at line {}: {}", line_num + 1, e))?;

        let timestamp = parts[1]
            .trim()
            .parse::<f64>()
            .map_err(|e| format!("Invalid timestamp at line {}: {}", line_num + 1, e))?;

        let value = parts[2]
            .trim()
            .parse::<f64>()
            .map_err(|e| format!("Invalid value at line {}: {}", line_num + 1, e))?;

        data_points.push(DataPoint {
            index,
            timestamp,
            value,
        });
    }

    if data_points.is_empty() {
        return Err("No valid data points found in CSV".to_string());
    }

    Ok(data_points)
}

/// Create a prediction request from CSV data
pub fn create_prediction_request(
    csv_buffer: &str,
    port: String,
    baud_rate: u32,
    collection_duration_ms: u64,
) -> Result<PredictionRequest, String> {
    let data = parse_csv_data(csv_buffer)?;
    let row_count = data.len();

    let dataset_id = uuid::Uuid::new_v4().to_string();
    let timestamp = chrono::Utc::now().to_rfc3339();

    Ok(PredictionRequest {
        dataset_id,
        timestamp,
        row_count,
        data,
        metadata: DatasetMetadata {
            port,
            baud_rate,
            collection_duration_ms,
        },
    })
}

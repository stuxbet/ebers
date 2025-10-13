use sea_orm::DatabaseConnection;
use tokio::sync::Mutex;

// Re-export shared types for convenience
pub use shared_types::{DetectionResult, Patient, Test, TestStatus, TestWithPatient};

/// Type alias for the database pool state
pub type DbState = Mutex<DatabaseConnection>;

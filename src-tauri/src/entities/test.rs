use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "tests")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub uuid: String,
    pub patient_id: i64,
    pub test_type: String,
    pub device_id: Option<String>,
    pub firmware_version: Option<String>,
    pub detection_result: Option<String>,
    pub confidence: Option<f64>,
    pub raw_response: Option<String>,
    pub status: String,
    pub error_message: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub completed_at: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::patient::Entity",
        from = "Column::PatientId",
        to = "super::patient::Column::Id"
    )]
    Patient,
}

impl Related<super::patient::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Patient.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}


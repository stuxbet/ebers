use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "patients")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub uuid: String,
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: Option<String>,
    pub patient_id_number: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::test::Entity")]
    Tests,
}

impl Related<super::test::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tests.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}


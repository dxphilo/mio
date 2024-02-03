use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "links")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,
    pub link: String,
    pub count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
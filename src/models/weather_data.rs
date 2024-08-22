use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "WeatherStats")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub zip: String,
    pub city: String,
    pub temperature: f64,
    pub weather: String,
    pub humidity: String,
    pub wind_speed: f64,
    pub measurement_time: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No Relation")
    }
}

impl ActiveModelBehavior for ActiveModel {}

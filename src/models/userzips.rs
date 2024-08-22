use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "UserZips")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Char(Some(10))")]
    pub zip_code: String,
    #[sea_orm(primary_key)]
    pub user_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Users,
    TargetZips,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Users => Entity::belongs_to(super::users::Entity)
                .from(Column::UserId)
                .to(super::users::Column::Id)
                .into(),
            Self::TargetZips => Entity::belongs_to(super::targetzips::Entity)
                .from(Column::ZipCode)
                .to(super::targetzips::Column::ZipCode)
                .into(),
        }
    }
}


impl ActiveModelBehavior for ActiveModel {}

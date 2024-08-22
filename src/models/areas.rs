use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "Areas")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    TargetZips,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::TargetZips => Entity::has_many(super::targetzips::Entity).into(),
        }
    }
}

impl Related<super::targetzips::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TargetZips.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

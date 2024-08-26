use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "TargetZips")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Char(Some(10))")]
    pub zip_code: String,
    pub area_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Areas,
    UserZips,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Areas => Entity::belongs_to(super::areas::Entity)
                .from(Column::AreaId)
                .to(super::areas::Column::Id)
                .into(),
            Self::UserZips => Entity::has_many(super::users::Entity).into(),
        }
    }
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        super::userzips::Relation::Users.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::userzips::Relation::TargetZips.def().rev())
    }
}

impl Related<super::areas::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Areas.def()
    }
}


impl ActiveModelBehavior for ActiveModel {}

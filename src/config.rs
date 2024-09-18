

//! This module is responsible for loading the configuration from the database.
//! 
//! The module includes a function to retrieve the target zip codes from the database and a struct to hold the data.
pub mod config {
    use sea_orm::DatabaseConnection;
    use sea_orm::EntityTrait;
    use sea_orm::QuerySelect;
    
 
    use crate::models::targetzips;

    use log::{info, debug};

    pub struct Zips {
        pub zips:Vec<String>,
    }

  
    /// Load the target zip codes from the database.
    /// 
    /// Uses a database connection and sea_orm model to retrieve the data.
    pub async fn load_config_db(db: &DatabaseConnection) -> Result<Zips, sea_orm::DbErr> {
        info!("Getting target zips from the database");
        let zips = targetzips::Entity::find()
            .select_only()
            .column(targetzips::Column::ZipCode)
            .into_tuple::<String>()
            .all(db)
            .await?;

        info!("Target zips retrieved from the database");
        debug!("{:?}", zips);

        Ok(Zips{zips})
            
    }




}
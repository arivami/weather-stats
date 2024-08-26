


pub mod config {
    use sea_orm::DatabaseConnection;
    use sea_orm::EntityTrait;
    use sea_orm::QuerySelect;
    
 
    use crate::models::targetzips;

    pub struct Zips {
        pub zips:Vec<String>,
    }

  

    pub async fn load_config_db(db: &DatabaseConnection) -> Zips {
        // get everything from the TargetZips table
        println!("Getting target zips from the database");
        let zips = targetzips::Entity::find()
            .select_only()
            .column(targetzips::Column::ZipCode)
            .into_tuple::<String>()
            .all(db)
            .await
            .expect("Failed to retrieve zip codes from the TargetZips table");

        println!("Target zips retrieved from the database");
        println!("{:?}", zips);

        Zips{zips}
            
    }




}
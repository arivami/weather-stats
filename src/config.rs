


pub mod config {
    use sea_orm::DatabaseConnection;
    use sea_orm::EntityTrait;
    use sea_orm::QuerySelect;
    
    use serde_json;
    use serde::{Deserialize, Serialize};
    use std::fs;
    use rand::Rng;

    use crate::models::targetzips;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct AreaCluster{
        pub area_name:String,
        pub zips:Vec<String>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct WeatherPullConf {
        pub targets:Vec<AreaCluster>,
        pub time_offsets:Vec<i32>,
    }

    pub struct Zips {
        pub zips:Vec<String>,
    }

    pub fn load_config(file_name:String) -> WeatherPullConf {
        let contents = fs::read_to_string(file_name).expect("cannot read file");
        
        let deserialized: WeatherPullConf = serde_json::from_str(&contents).unwrap();
        deserialized
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


    pub fn randomize_target_list(pull_conf:WeatherPullConf)->Vec<String>{
        let iter = pull_conf.targets.iter();
        iter.map(|x| -> String {
            let l = x.zips.len();
            match l {
                0=> "00000".to_string(),
                1=> x.zips[0].to_string(),
                _=> x.zips[rand::thread_rng().gen_range(0..=l-1)].to_string()
            }           
        }
        ).collect()
    }

}
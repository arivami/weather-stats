use weather_stats::config::config::*;
use std::fs;
use log::error;

use sea_orm::{
    entity::prelude::*, entity::*,
    DatabaseBackend, MockDatabase, MockExecResult, MockRow
};

use sea_orm::DatabaseConnection;
use sea_orm::EntityTrait;
use sea_orm::QuerySelect;

use weather_stats::models::weather_data;
use weather_stats::models::targetzips;





#[tokio::test]
#[ignore]
async fn test_load_config_db() {
    let db:DatabaseConnection = MockDatabase::new(DatabaseBackend::MySql)
    .append_query_results([
        vec![targetzips::Model {
            zip_code: "95124".to_string(),
            area_id: 1,
        },
        targetzips::Model {
            zip_code: "95014".to_string(),
            area_id: 2,
        }],
    ])
    .into_connection();

   
    
    
    let result = load_config_db(&db).await.unwrap();
    // if result.is_err() {
    //     println!("Failed to load target zips from the database: {:?}", result.err());
    // }
    
    let expected = Zips {
        zips: vec!["95124".to_string(), "95014".to_string()],
    };

    assert_eq!(result.zips, expected.zips);
}
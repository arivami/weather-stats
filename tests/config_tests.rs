use weather_stats::config::config::*;
use std::fs;

fn setup() -> WeatherPullConf {
    let result = WeatherPullConf {
        targets: vec![
            AreaCluster {
                area_name: "San Jose".to_string(),
                zips: vec!["95124".to_string(), "95014".to_string(), "95123".to_string()],
            },
            AreaCluster {
                area_name: "Los Angeles".to_string(),
                zips: vec!["90046".to_string(), "90291".to_string()],
            },
        ],
        time_offsets: vec![4, 8, 12, 16, 20, 24],
    };
    result
}




#[test]
fn test_load_config() {
    let expected = setup();
    let temp_dir = std::env::temp_dir();
    let temp_file_path = temp_dir.join("test_config.json");
    let file = r#"
    {
        "targets": [
        {
            "area_name": "San Jose",
            "zips": ["95124", "95014", "95123"]
        },
        {
            "area_name": "Los Angeles",
            "zips": ["90046", "90291"]
        }],
        "time_offsets": [4, 8, 12, 16, 20, 24]
    }
    "#;

    fs::write(&temp_file_path, file).expect("Failed to write to temp file");

    let result = load_config(temp_file_path.to_str().unwrap().to_string());

    assert_eq!(result.targets[0].area_name, expected.targets[0].area_name);
    assert_eq!(result.targets[0].zips, expected.targets[0].zips);
    assert_eq!(result.targets[1].area_name, expected.targets[1].area_name);
    assert_eq!(result.targets[1].zips, expected.targets[1].zips);
    assert_eq!(result.time_offsets, expected.time_offsets);
}


#[test]
fn test_randomize_target_list() {
    let area1 = vec!["95124".to_string(), "95014".to_string(), "95123".to_string()];
    let area2 = vec!["90046".to_string(), "90291".to_string()];
    let result = randomize_target_list(setup());

    assert_eq!(result.len(), 2);
    assert_eq!((area1.contains(&result[0]) && area2.contains(&result[1])) || (area2.contains(&result[0]) && area1.contains(&result[1])), true);

}
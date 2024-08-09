mod common;

use config;


#[test]
fn test_load_config() {
    let pull_conf = config::config::load_config("/workspace/src/test-data/weather-pull-conf.json".to_string());
    assert_eq!(pull_conf.target_list.len(), 2);
    assert_eq!(pull_conf.target_list[0].zip, "12345");
    assert_eq!(pull_conf.target_list[0].url, "http://api.openweathermap.org/data/2.5/weather");
    assert_eq!(pull_conf.target_list[1].zip, "54321");
    assert_eq!(pull_conf.target_list[1].url, "http://api.openweathermap.org/data/2.5/weather");
}


#[test]
fn test_randomize_target_list() {
    let mut pull_conf = config::config::load_config("/workspace/src/test-data/weather-pull-conf.json".to_string());
    let mut target_list = pull_conf.target_list.clone();
    let randomized_list = config::config::randomize_target_list(&mut target_list);
    assert_eq!(randomized_list.len(), 2);
    assert_eq!(randomized_list[0].zip, "54321");
    assert_eq!(randomized_list[0].url, "http://api.openweathermap.org/data/2.5/weather");
    assert_eq!(randomized_list[1].zip, "12345");
    assert_eq!(randomized_list[1].url, "http://api.openweathermap.org/data/2.5/weather");
}
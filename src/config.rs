


pub mod config {
    use serde_json;
    use serde::{Deserialize, Serialize};
    use std::fs;
    use rand::Rng;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct CityCluster{
        city_name:String,
        zips:Vec<String>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct WeatherPullConf {
        targets:Vec<CityCluster>,
        time_offsets:Vec<i32>,
    }

    pub fn load_config(file_name:String) -> WeatherPullConf {
        let contents = fs::read_to_string(file_name).expect("cannot read file");
        
        let deserialized: WeatherPullConf = serde_json::from_str(&contents).unwrap();
        deserialized
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
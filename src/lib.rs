use std::env;
use yaml_rust::{YamlLoader, Yaml};
use std::fs;

fn get_param_file(path: String, node_name: &str) -> Yaml{
    let file = fs::read_to_string(path);
    let s = file.unwrap().to_string();
    let docs = YamlLoader::load_from_str(&s).unwrap();
    let doc = &docs[0];

    if !doc[node_name]["ros__parameters"].is_badvalue(){
        doc[node_name]["ros__parameters"].clone()
    }else if !doc["/**"]["ros__parameters"].is_badvalue(){
        doc["/**"]["ros__parameters"].clone()
    }else{
        Yaml::BadValue
    }
}

fn get_param(node_name: &str, key: &str) -> Yaml{
    let args: Vec<String> = env::args().collect();
    if args.iter().find(| &x| x == "--ros-args") == None {
        return Yaml::BadValue;
    }
    for (i, arg) in args.iter().enumerate(){
        if arg == "--params-file" {
            let param = get_param_file(args[i + 1].clone(), node_name);
            if !param[key].is_badvalue(){
                return param;
            }
        }
    }
    return Yaml::BadValue;
}

pub fn get_i64_parameter(node_name: &str, key: &str, defalt: i64)->i64{
    match get_param(node_name, key)[key].as_i64() {
        Some(value) => {
            return value;
        },
        None => {
            return defalt;
        }
    }
}

pub fn get_f64_parameter(node_name: &str, key: &str, defalt: f64)->f64{
    match get_param(node_name, key)[key].as_f64() {
        Some(value) => {
            return value;
        },
        None => {
            return defalt;
        }
    }
}

pub fn get_bool_parameter(node_name: &str, key: &str, defalt: bool)->bool{
    match get_param(node_name, key)[key].as_bool() {
        Some(value) => {
            return value;
        },
        None => {
            return defalt;
        }
    }
}

pub fn get_str_parameter(node_name: &str, key: &str, defalt: &str) -> String{
    match get_param(node_name, key)[key].as_str() {
        Some(value) => {
            return value.to_string();
        },
        None => {
            return defalt.to_string();
        }
    }
}
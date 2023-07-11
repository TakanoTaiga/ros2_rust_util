use safe_drive::{
    error::DynError, logger::Logger, pr_info
};
use std::env;
use std::fs;

fn get_launch_param(name: &str, value: &str) -> String{
    let args: Vec<String> = env::args().collect();

    let mut raw_lists: Vec<String> = vec!["".to_string()];

    for (i , arg) in args.iter().enumerate() {
        if arg != "--params-file" {
            continue;
        }
        match fs::read_to_string(args[i + 1].clone()){
            Ok(content) =>{
                let mut var = String::from("");
                for c in content.chars() {
                    if c != ':'{
                        if c != '\n' && c != ' '{
                            var.push(c);
                    }
                    }else{
                        if var != "ros__parameters" && var != "/**"{
                            raw_lists.push(var.clone());
                        }
                        var = String::from("");
                    }
                }
                raw_lists.push(var.clone());
            }
            Err(_) => {
                return value.to_string();
            }
        }
    }

    for (i , list_name) in raw_lists.iter().enumerate() {
        if name.to_string() == *list_name {
            return raw_lists[i + 1].clone();
        }
    }

    return value.to_string();
}

fn get_remap(topic_name: &str) -> String{
    let args: Vec<String> = env::args().collect();

    let mut remap_list: Vec<String> = vec!["".to_string()];

    for (i , arg) in args.iter().enumerate() {
        if arg != "-r" {
            continue;
        }
        remap_list.push(args[i + 1].clone());
    }

    for r in remap_list.iter() {
        let r_sp: Vec<String> = r.split(":=").map(|s| s.to_string()).collect();
        if r_sp[0] == topic_name {
            return r_sp[1].clone();
        }
    }

    return topic_name.to_string();
}

fn main() -> Result<(), DynError> {
    // Create a logger.
    let logger: Logger = Logger::new("main");

    pr_info!(logger,"param:{:?}", get_launch_param("test" , "12"));
    pr_info!(logger,"remap:{:?}", get_remap("string_topic"));

    pr_info!(logger,"param:{:?}", get_launch_param("abc_param" , "12"));
    pr_info!(logger,"remap:{:?}", get_remap("abc_topic"));
    
    Ok(())
}

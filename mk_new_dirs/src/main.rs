use clap::{command,Arg};
use std::fs;

fn main(){
    let matches = command!("mk_new_dirs")
        .version("1.0")
        .author("kenf1")
        .about("Create new directories in format of W_S_")
        .arg(
            Arg::new("target_dir")
                .short('t')
                .value_name("TARGET")
                .help("Directory where folders will be created.")
                .required(true)
        )
        .arg(
            Arg::new("total_wks")
                .short('w')
                .value_name("WEEK")
                .help("Total number of weeks in block.")
                .required(true)
        )
        .get_matches();

    //get target directory
    let arg_dir = matches.get_one::<String>("target_dir")
        .unwrap()
        .to_string();
    // println!("Entered path: {}",dir); //debug arg_dir

    //get total weeks (parse as string -> u8)
    let temp_wks = matches.get_one::<String>("total_wks")
        .unwrap()
        .to_string();
    let arg_wks = temp_wks
        .parse::<u8>()
        .unwrap();
    // println!("Entered weeks: {}",arg_wks); //debug arg_wks

    //create folder by wk then session
    for i in 1..=arg_wks{
        for j in 1..=5{ //assume 5 sessions per wk
            fs::create_dir(format!("{}/W{}S{}",arg_dir,i,j))
                .unwrap();
        }
    }

    //exit message
    println!("Successfully created folders for: Weeks 1-{} in {}",arg_wks,arg_dir);
}
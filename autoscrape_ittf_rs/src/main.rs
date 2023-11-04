#![allow(unused_imports,dead_code)]

use reqwest::Client;
use scraper::{Html,Selector};
use std::{fs::File,io::Write};

mod all_fn;

struct AllInfo{
    url: String,
    csv_path: String,
    filt_tag: String
}

#[tokio::main]
async fn main() -> Result<(),reqwest::Error>{
    let info = AllInfo{
        url: "https://www.ittf.com/wp-content/uploads/2023/10/2023_41_SEN_MS.html".to_string(),
        csv_path: "./Output/untidy_output.csv".to_string(),
        filt_tag: "#content > table > tbody > tr.rrow.age-SEN".to_string()
    };

    //return html
    let raw_html = all_fn::get_html(&info.url).await?;
    // all_fn::debug_fn(&raw_html); //debug

    /*
        get values by table rows
            .concat used to rm outer vector
            tr:nth-child increments by odd int
    */
    let temp_data = all_fn::get_value(&raw_html,&info.filt_tag);
    // all_fn::debug_fn(&temp_data); //debug

    /*
        convert vector of vectors -> csv-like format
            go thru each row & concat w/ `,`
            combine & concat w/ `\n`
    */
    let txt_contents = temp_data
        .iter()
        .map(|row|row.join(","))
        .collect::<Vec<String>>()
        .join("\n");
    
    //create untidy csv
    let mut file = File::create(&info.csv_path)
        .expect("Unable to create file.");
    
    file.write_all(txt_contents.as_bytes())
        .expect("Unable to write to file.");

    println!("Success");
    
    Ok(())
}
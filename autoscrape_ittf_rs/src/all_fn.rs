use reqwest::Client;
use scraper::{Html,Selector};

//get + return html for parsing
pub async fn get_html(url: &str) -> Result<Html,reqwest::Error>{
    let body = reqwest::Client::new()
        .get(url)
        .send()
        .await?
        .text()
        .await?;
    let raw_html = Html::parse_document(&body); //convert body to html object (parse prep)
    
    Ok(raw_html)
}

//obtain all values for a specific var -> vector of &str
pub fn get_value<'a>(html_file: &'a Html,tag: &'a str) -> Vec<Vec<&'a str>>{
    let mut values_vec = Vec::new();
    let temp_selector = Selector::parse(&tag).unwrap();
    
    //push to values_vec vector
    for item in html_file.select(&temp_selector){
        let temp_val = item.text().collect::<Vec<_>>();
        values_vec.push(temp_val);
    }
    
    return values_vec;
}

//generic function to print input
pub fn debug_fn<T>(fn_input: &T) where T: std::fmt::Debug{
    println!("{:?}",fn_input);
}
use clap::{command,Arg};
use yaqr;

fn main(){
    let matches = command!("yaqr_cli")
        .version("1.0")
        .author("kenf1")
        .about("Yet another QR code reader - CLI version")
        .arg(
            Arg::new("Source")
                .short('s')
                .long("QR code source")
                .value_name("SOURCE")
                .help("Will QR code image be passed into app via url or local path?\nAccepted values are: `url` or `local`.")
                .required(true)
        )
        .arg(
            Arg::new("Location")
                .short('l')
                .long("QR code path")
                .value_name("LOCATION")
                .help("URL/PATH to QR code")
                .required(true)
        )
        .get_matches();
    
    //parse args
    if matches.args_present(){
        let src = matches
            .get_one::<String>("Source")
            .unwrap()
            .to_string();

        let loc = matches
            .get_one::<String>("Location")
            .unwrap()
            .to_string();

        match src.as_str(){
            "url" => yaqr::from_remote(&loc),
            "local" => yaqr::from_remote(&loc),
            _ => panic!("Accepted values are: 'url' or 'local'. Please try again.")
        }
    }
}
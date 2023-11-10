use tts_rust::{
    tts::GTTSClient,
    languages::Languages
};

fn main(){
    let narrator: GTTSClient = GTTSClient{
        volume: 1.0, //max = 1.0
        language: Languages::English,
        tld: "com",
    };
    let _ = narrator.speak("Hello world");
}
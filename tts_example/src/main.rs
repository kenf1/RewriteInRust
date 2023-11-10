use tts::*;

fn main() -> Result<(),Error>{
    let mut tts = Tts::default()?;

    tts.set_volume(tts.max_volume())?;
    tts.speak("Hello world!",true)?;

    Ok(())
}
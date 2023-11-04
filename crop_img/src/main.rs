use yaqr; //includes image::DynamicImage

fn main(){
    //get image from url
    let url = "https://ih1.redbubble.net/image.4406644318.0421/st,small,507x507-pad,600x600,f8f8f8.jpg";
    let image = yaqr::image_from_url(url).unwrap();

    //crop & save
    let cropped = image.crop_imm(130,69,335,458);
    cropped.save("../ref/rewrite.jpg").unwrap();
}
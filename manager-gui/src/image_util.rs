pub type ImageMap = conrod_core::image::Map<glium::texture::Texture2d>;
pub type ImageId = conrod_core::image::Id;

use overwatch::Hero;
use std::collections::BTreeMap;
use std::path;

const OVERWATCH_PORTRAITS: &str = "images/overwatch/portraits/";

pub fn load_ow_portraits(
    image_map: &mut ImageMap,
    display: &glium::Display,
) -> BTreeMap<Hero, ImageId> {
    let assets = find_folder::Search::KidsThenParents(3, 5)
        .for_folder("assets")
        .unwrap();

    let portrait_dir = assets.join(OVERWATCH_PORTRAITS);
    overwatch::Hero::iter()
        .map(|h| {
            let mut hero_portrait = portrait_dir.clone();
            hero_portrait.push(h.blizzard_name());
            hero_portrait.set_extension("png");
            //let img = conrod_core::image::open(hero_portrait).unwrap();
            let img = load_image(display, hero_portrait);
            let id = image_map.insert(img);
            (h, id)
        })
        .collect()
}

// Load an image from our assets folder as a texture we can draw to the screen.
fn load_image<P>(display: &glium::Display, path: P) -> glium::texture::Texture2d
where
    P: AsRef<std::path::Path>,
{
    let path = path.as_ref();
    let rgba_image = image::open(&std::path::Path::new(&path)).unwrap().to_rgba();
    let image_dimensions = rgba_image.dimensions();
    let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(
        &rgba_image.into_raw(),
        image_dimensions,
    );
    //let texture = glium::texture::SrgbTexture2d::new(display, raw_image).unwrap();
    let texture = glium::texture::Texture2d::new(display, raw_image).unwrap();
    texture
}

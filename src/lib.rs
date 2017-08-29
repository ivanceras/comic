extern crate memenhancer;
extern crate svgbob;
extern crate svg;

use svg::node::Node;
use svg::node::Text as TextNode;
use svg::node::element::SVG;
use svgbob::{Grid,Settings};


/// using default text size, 8.0x16.0 px
pub fn to_svg(input:&str) -> SVG {
    to_svg_with_textsize(input, 8.0, 16.0)
}
///
///    let text_width:f32 = 8.0;
///    let text_height:f32 = 16.0;
///
pub fn to_svg_with_textsize(input: &str, text_width: f32, text_height: f32) -> SVG {
    let (svg_memes, updated_input) = memenhancer::get_meme_svg(input, text_width, text_height); 
    let mut settings = Settings::default();
    settings.set_size(text_width, text_height);
    let grid = Grid::from_str(&updated_input, &settings); 
    let mut svg = grid.get_svg();
    for meme in svg_memes{
        let text_node = TextNode::new(meme.to_string());
        svg.append(text_node);
    }
    svg
}

pub fn to_svg_with_textsize_nooptimization(input: &str, text_width: f32, text_height: f32) -> SVG {
    let (svg_memes, updated_input) = memenhancer::get_meme_svg(input, text_width, text_height); 
    let mut settings = Settings::default();
    settings.set_size(text_width, text_height);
    let grid = Grid::from_str(&updated_input, &settings); 
    let mut svg = grid.get_svg();
    for meme in svg_memes{
        let text_node = TextNode::new(meme.to_string());
        svg.append(text_node);
    }
    svg
}


#[test]
fn test_svg(){
    let text_width:f32 = 8.0;
    let text_height:f32 = 16.0;
    let input = "─=≡Σ( ͡° ͜ʖ ͡°)";

    let out = to_svg_with_textsize(input, text_width, text_height);
    println!("{}", out);
    panic!();
}

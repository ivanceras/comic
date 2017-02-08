extern crate memenhancer;
extern crate svgbob;
extern crate svg;

use svg::node::Node;
use svg::node::Text as TextNode;
use svg::node::element::SVG;

///
///    let text_width:f32 = 8.0;
///    let text_height:f32 = 16.0;
///
pub fn to_svg_with_textsize(input: &str, text_width: f32, text_height: f32) -> SVG {
    let (svg_memes, updated_input) = memenhancer::get_meme_svg(input, text_width, text_height); 
    let mut svg = svgbob::to_svg_with_size(&updated_input, text_width, text_height);
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

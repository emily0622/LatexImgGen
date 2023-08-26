#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
extern crate mathjax;
use LatexImgGen::mathjax::MathJax;
use std::fs;


pub fn render(full_img_name: String, eq_input: String) -> String {
    let end_index = full_img_name.len()-4;
    let img_name: String;
    let new_edition_num: i32;
    if (end_index < 6) || (&full_img_name[end_index..end_index+4] != ".svg"){
        // the full_img_name has no editions yet. It is in the form img_name
        // without a ".svg" or "_edition"
        img_name = full_img_name;
        new_edition_num = 0;

    } else {
        let edition_index: usize;
        (new_edition_num,edition_index) = FindEditionNum(full_img_name.clone());
        img_name = full_img_name[0..edition_index].to_string();
    }

    let new_full_img_name = ProduceImage(new_edition_num, eq_input, img_name);
    return new_full_img_name;
 }

 fn FindEditionNum(full_img_name: String) -> (i32,usize) {
    let end_index = full_img_name.len()-4;
    let mut edition_index = 0;
    for (i, c) in full_img_name[0..end_index].chars().enumerate() {
        // [0..-4] to avoid the ".svg" at the end
        // do something with character `c` and index `i`
        // this function isolates the edition number of the image
        // e.g. math2latex_3.svg -> 3 ; robberproblem2_4.svg -> 4
        if c == '_' {
            edition_index = i;
        }
    }
    let edition_num_string = full_img_name[edition_index+1..end_index].to_string();
    let edition_num = edition_num_string.parse::<i32>().unwrap();
    let new_edition_num = edition_num + 1;
    return (new_edition_num,edition_index);
 }

 fn ProduceImage(edition_num: i32, eq_input: String, img_name: String) -> String {
    let suffix = ".svg";
    let mut full_img_name = img_name.clone().to_owned();
    full_img_name.push_str("_");
    full_img_name.push_str(&edition_num.to_string());
    full_img_name.push_str(suffix);

    let mut prev_img_name: String = "test".to_owned();
    prev_img_name.push_str(&(edition_num-1).to_string());
    prev_img_name.push_str(suffix);
    //render new image
    render_eq(&eq_input, &full_img_name);
    let let_ = fs::remove_file(prev_img_name);
    return full_img_name;
}


fn render_eq(input: &str, img_name: &str) {
    let renderer = MathJax::new().unwrap();
    let result = renderer.render(input).unwrap();
    let svg_string = result.into_raw(); // This is a `<svg></svg>` element.
    std::fs::write(img_name, svg_string).unwrap();
}



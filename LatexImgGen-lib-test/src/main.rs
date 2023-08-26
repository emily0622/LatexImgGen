#![allow(non_snake_case)]
extern crate LatexImgGen_lib;
// use LatexImgGen_lib::LatexImgGen::play;
use LatexImgGen_lib::LatexImgGen;
// use dioxus::prelude::*;
use std::assert;
use std::path::Path;

pub fn main() {
    println!("Starting Test 0");
    test0();

    println!("Starting Test 1");
    test1();

    println!("Starting Test 2");
    test2();
    // println!("Starting Test 2");

//    test1();
//    println!("Test2");
//    play("Tutorialspoint".to_string())
}


pub fn test0() {
    let eq = "y=\\frac{1}{x}".to_string();
    let name = LatexImgGen::render("test0".to_string(),eq,None,true);
    assert!(name == "test0_0.svg".to_string(),"Incorrect Img Name, Rendered: {}; should be: test0_0.svg",name);
    println!("TEST0 PASSED");
}

pub fn test1() {
    let mut name = "test1".to_string();
    name = LatexImgGen::render(name,"y=\\frac{1}{x}".to_string(),None,false);
    assert!(name == "test1_0.svg".to_string(),"{}","Incorrect Img Name, Rendered: {name}; should be: test1_0.svg");
    name = LatexImgGen::render(name,"y=\\frac{2}{x}".to_string(),None,true);
    assert!(name == "test1_1.svg".to_string(),"{}","Incorrect Img Name, Rendered: {name}; should be: test1_1.svg");
    assert!(!(Path::new("../test1_0.svg").exists()),"Prev image not removed");
    name = LatexImgGen::render(name,"y=\\frac{123}{x}".to_string(),None,true);
    assert!(name == "test1_2.svg".to_string(),"{}","Incorrect Img Name, Rendered: {name}; should be: test1_2.svg");
    assert!(!(Path::new("../test1_1.svg").exists()),"Prev image not removed");
        
    println!("TEST1 PASSED");
}

pub fn test2() {
    let mut name = "test2".to_string();
    let path = "../imgfolder/";
    name = LatexImgGen::render(name,"y=\\frac{1}{x}".to_string(),Some(path),true);
    assert!(name == "test2_0.svg".to_string(),"{}","Incorrect Img Name, Rendered: {name}; should be: test2_0.svg");
    assert!(Path::new("../imgfolder/test2_0.svg").exists(),"Image not found in correct filepath");
    name = LatexImgGen::render(name,"y=\\frac{2}{x}".to_string(),Some(path),true);
    assert!(name == "test2_1.svg".to_string(),"{}","Incorrect Img Name, Rendered: {name}; should be: test2_1.svg");
    assert!(Path::new("../imgfolder/test2_1.svg").exists(),"Image not found in correct filepath");
    assert!(!(Path::new("../imgfolder/test2_0.svg").exists()),"Prev image not removed");
    println!("TEST2 PASSED");
}


// pub fn test2() {
//     dioxus_desktop::launch(App);
// }

    
// fn App(cx: Scope) -> Element {

//     let img_name = use_state(cx, || "test0.svg".to_string());
//     let eq_input = use_state(cx, || "y=\\frac{1}{x}".to_string());
//     let mut img_bool = use_state(cx, || 0);

//     cx.render(rsx!{
//         form {
//             onsubmit: move |_| {
//                 img_name.set(LatexImgGen::render(img_name,eq_input.to_string()));
//                 img_bool.set(1);
//                 },
//             input {
//                 value: "{eq_input}",
//                 oninput: |event| eq_input.set(event.value.clone()),
//                 println!("eq_input: {eq_input}"),
//             }
//             input {
//                 r#type: "submit",
//             }
//         },
//         if *img_bool == 1 {
//             img { src: "{img_name}" }
//         } 
//     })

// }

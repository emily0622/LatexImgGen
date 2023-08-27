# LatexImgGen

A rust crate for generating Latex images of equations with an edition number to avoid requiring cache flushing and provides more context than timestamp attributes. Previous editions of the image can be kept when the _del_prev parameter is set to false for keeping a record of prior images or removed when the _del_prev parameter is set to false. 

### Parameter Inputs
```
render(full_img_name: String, eq_input: String, filepath: Option<&str>, _del_prev: bool)
```
full_img_name -> name of image. Must not end with a '_' and should not include .svg \
eq_input -> a latex equation. Can write expressions in quotations or like so: let expression = r#"y=\frac{1}{x}"#; \
filepath -> add relative path using ../dir/ or use None to add into current directory \
_del_prev -> true if you want the previous version to be deleted from the directory \

### Example
```
let mut name = "test".to_string();
let mut eq = "y=\\frac{1}{x}".to_string();
let path = "../imgfolder/";
name = LatexImgGen::render(name, eq, Some(path), true);
```
an image called test_0.svg will be generated in the imgfolder dir
```
eq = "y=\\frac{1}{1000}".to_string();
name = LatexImgGen::render(name,eq,Some(path),true);
```
an image called test_1.svg will be generated in the imgfolder dir

### Dioxus Example

In this example, an input area is given for the user to submit a latex equation. Once the first equation is submitted, the image is created as eq_img_0.svg. Every future submission regenerates the image with an incremented edition.

```
#![allow(non_snake_case)]
use dioxus::prelude::*;
use LatexImgGen_lib::LatexImgGen;

fn main() {
    dioxus_desktop::launch(App);
}

    
fn App(cx: Scope) -> Element {

    let img_name = use_state(cx, || "eq_img".to_string());
    let eq_input = use_state(cx, || "y=\\frac{1}{x}".to_string());
    let mut img_bool = use_state(cx, || 0);

    cx.render(rsx!{
        form {
            onsubmit: move |_| {
                img_name.set(LatexImgGen::render(img_name.to_string(),eq_input.to_string(),None,true));
                img_bool.set(1);
                },
            input {
                value: "{eq_input}",
                oninput: |event| eq_input.set(event.value.clone()),
            }
            input {
                r#type: "submit",
            }
        },

        ImgDisplay {
            imgbool: **img_bool,
            imgname: (**img_name).to_string(),
        }
    })


}

fn ImgDisplay(cx: Scope<ImgBool>) -> Element {
    if cx.props.imgbool == 1 {
        cx.render(rsx! {
            img {
                src: "{cx.props.imgname}"
            }
        })
    } else {
        None
    }
}


#[derive(PartialEq, Props)]
struct ImgBool {
    imgbool: i32,
    imgname: String,
}
```




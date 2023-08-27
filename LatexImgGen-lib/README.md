# LatexImgGen

A rust crate for generating Latex images of equations with an edition number to avoid cache flushing and timestamp attributes

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






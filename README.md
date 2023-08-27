# LatexImgGen

A rust crate for generating Latex images of equations with an edition number to avoid cache flushing and timestamp attributes


### Example
\begin{lstlisting}

let mut name = "test".to_string();
let path = "../imgfolder/";
name = LatexImgGen::render(name,"y=\\frac{1}{x}".to_string(),Some(path),true);
// an image called test_0.svg will be generated
name = LatexImgGen::render(name,"y=\\frac{2}{x}".to_string(),Some(path),true);
// an image called test_1.svg will be generated
\end{lstlisting}

### Parameter Inputs
\begin{lstlisting}
render(full_img_name: String, eq_input: String, filepath: Option<&str>, _del_prev: bool)
\end{lstlisting}

filepath -> if you want the image added to your current directory, use None
_del_prev -> true if you want the previous version to be deleted

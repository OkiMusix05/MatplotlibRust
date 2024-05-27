use inline_python::{Context, python};
fn main() {
    let c:Context = python!{
        import matplotlib.pyplot as plt;
        import numpy as np;
        plt.rcParams["text.usetex"] = True;
    };
    println!("Rust supercharged with Python and Matplotlib");
}

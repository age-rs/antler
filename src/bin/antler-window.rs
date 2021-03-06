//! Rendering engine binary.

use antler::{
    input::{Settings, Shader, ShaderBuilder},
    parts::{Attributes, Camera, CameraBuilder, Scene},
    run::window_thread,
};
use arctk::{
    args,
    file::{Build, Load, Redirect, Save},
    geom::{Mesh, MeshBuilder, Tree, TreeBuilder},
    img::GradientBuilder,
    ord::Set,
    util::{banner, dir, gradient},
};
use arctk_attr::input;
use palette::{Gradient, LinSrgba};
use std::{
    env::current_dir,
    path::{Path, PathBuf},
};

/// Key type.
type Key = String;

/// Input parameters.
#[input]
struct Parameters {
    /// Adaptive mesh settings.
    tree: Redirect<TreeBuilder>,
    /// Render runtime settings.
    sett: Redirect<Settings>,
    /// Surfaces set.
    surfs: Redirect<Set<Key, MeshBuilder>>,
    /// Attributes set.
    attrs: Redirect<Set<Key, Attributes>>,
    /// Colour set.
    cols: Redirect<Set<Key, GradientBuilder>>,
    /// Shader.
    shader: Redirect<ShaderBuilder>,
    /// Camera.
    cam: Redirect<CameraBuilder>,
    /// Pixel update size.
    update_size: u64,
}

fn main() {
    let term_width = arctk::util::term::width().unwrap_or(80);

    banner::title("RENDER - WINDOW", term_width);
    let (params_path, in_dir, out_dir) = init(term_width);
    let params = input(term_width, &in_dir, &params_path);
    let (tree_sett, render_sett, surfs, attrs, cols, shader, cam, update_size) =
        build(term_width, &in_dir, params);
    let tree = grow(term_width, tree_sett, &surfs);
    let input = Scene::new(&tree, &render_sett, &surfs, &attrs, &cols);
    banner::section("Rendering", term_width);

    let output =
        window_thread(update_size, &input, &shader, &cam).expect("Failed to perform rendering.");
    output
        .img
        .save(&out_dir.join("render.png"))
        .expect("Failed to save output data.");

    banner::section("Finished", term_width);
}

/// Initialise the command line arguments and directories.
fn init(term_width: usize) -> (PathBuf, PathBuf, PathBuf) {
    banner::section("Initialisation", term_width);
    banner::sub_section("Command line arguments", term_width);
    args!(bin_path: PathBuf;
        params_path: PathBuf
    );
    println!("{:>32} : {}", "binary path", bin_path.display());
    println!("{:>32} : {}", "parameters path", params_path.display());

    banner::sub_section("Directories", term_width);
    let cwd = current_dir().expect("Failed to determine current working directory.");
    let (in_dir, out_dir) = dir::io_dirs(Some(cwd.join("input")), Some(cwd.join("output")))
        .expect("Failed to initialise directories.");
    println!("{:>32} : {}", "input directory", in_dir.display());
    println!("{:>32} : {}", "output directory", out_dir.display());

    (params_path, in_dir, out_dir)
}

/// Load the input files.
fn input(term_width: usize, in_dir: &Path, params_path: &Path) -> Parameters {
    banner::section("Input", term_width);
    banner::sub_section("Parameters", term_width);
    let path = in_dir.join(params_path);

    Parameters::load(&path).expect("Failed to load parameters file.")
}

/// Build instances.
#[allow(clippy::type_complexity)]
fn build(
    term_width: usize,
    in_dir: &Path,
    params: Parameters,
) -> (
    TreeBuilder,
    Settings,
    Set<Key, Mesh>,
    Set<Key, Attributes>,
    Set<Key, Gradient<LinSrgba>>,
    Shader,
    Camera,
    u64,
) {
    banner::section("Building", term_width);
    banner::sub_section("Adaptive Tree Settings", term_width);
    let tree_sett = params
        .tree
        .build(in_dir)
        .expect("Failed to redirect adaptive tree settings.");

    banner::sub_section("Render Settings", term_width);
    let render_sett = params
        .sett
        .build(in_dir)
        .expect("Failed to redirect render settings.");

    banner::sub_section("Surfaces", term_width);
    let surfs = params
        .surfs
        .build(in_dir)
        .expect("Failed to redirect surfaces set.")
        .build(in_dir)
        .expect("Failed to build surfaces.");

    banner::sub_section("Attributes", term_width);
    let attrs = params
        .attrs
        .build(in_dir)
        .expect("Failed to redirect attributes set.");

    banner::sub_section("Colours", term_width);
    let cols = params
        .cols
        .build(in_dir)
        .expect("Failed to redirect colour gradients set.")
        .build(in_dir)
        .expect("Failed to build colour gradients set.");
    for (group, grad) in cols.map() {
        println!(
            "{:>32} : {}",
            &format!("[{}]", group),
            gradient::to_string(&grad, 32)
        );
    }

    banner::sub_section("Shader", term_width);
    let shader = params
        .shader
        .build(in_dir)
        .expect("Failed to redirect shader settings.")
        .build(in_dir)
        .expect("Failed to build shader.");

    banner::sub_section("Camera", term_width);
    let cam = params
        .cam
        .build(in_dir)
        .expect("Failed to redirect camera settings.")
        .build(in_dir)
        .expect("Failed to build build camera.");

    banner::sub_section("Update Size", term_width);
    let update_size = params.update_size;

    (
        tree_sett,
        render_sett,
        surfs,
        attrs,
        cols,
        shader,
        cam,
        update_size,
    )
}

/// Grow domains.
fn grow<'a>(term_width: usize, tree: TreeBuilder, surfs: &'a Set<Key, Mesh>) -> Tree<'a, &Key> {
    banner::section("Growing", term_width);

    banner::sub_section("Adaptive Tree", term_width);
    let tree = tree.build(&surfs);

    tree
}

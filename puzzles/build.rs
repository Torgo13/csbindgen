use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {        
    bindgen::Builder::default()
        //.header("c/puzzles/gtk.h")
        
        .header("c/puzzles/puzzles.h")
        .generate()
        .unwrap()
        .write_to_file("src/puzzles.rs")
        .unwrap();

    cc::Build::new()
        .files([
            "c/puzzles/windows.c",  // windows.cmake
            "c/puzzles/printing.c", // windows.cmake
            
            //"c/puzzles/nestedvm.c", // nestedvm.cmake
            //"c/puzzles/emcc.c",     // emscripten.cmake
            //"c/puzzles/gtk.c",      // unix.cmake
            
            "c/puzzles/blackbox.c",
            "c/puzzles/bridges.c",
            "c/puzzles/cube.c",
            "c/puzzles/dominosa.c",
            "c/puzzles/fifteen.c",
            "c/puzzles/filling.c",
            "c/puzzles/flip.c",
            "c/puzzles/flood.c",
            //"c/puzzles/fuzzpuzz.c",
            "c/puzzles/galaxies.c",
            "c/puzzles/guess.c",
            "c/puzzles/inertia.c",
            "c/puzzles/keen.c",
            "c/puzzles/lightup.c",
            //"c/puzzles/list.c",
            "c/puzzles/loopy.c",
            "c/puzzles/magnets.c",
            "c/puzzles/map.c",
            "c/puzzles/mines.c",
            "c/puzzles/mosaic.c",
            "c/puzzles/net.c",
            "c/puzzles/netslide.c",
            //"c/puzzles/no-icon.c",
            //"c/puzzles/nullfe.c",
            //"c/puzzles/nullgame.c",
            "c/puzzles/palisade.c",
            "c/puzzles/pattern.c",
            "c/puzzles/pearl.c",
            "c/puzzles/pegs.c",
            "c/puzzles/range.c",
            "c/puzzles/rect.c",
            "c/puzzles/samegame.c",
            "c/puzzles/signpost.c",
            "c/puzzles/singles.c",
            "c/puzzles/sixteen.c",
            "c/puzzles/slant.c",
            "c/puzzles/solo.c",
            "c/puzzles/tents.c",
            "c/puzzles/towers.c",
            "c/puzzles/tracks.c",
            "c/puzzles/twiddle.c",
            "c/puzzles/undead.c",
            "c/puzzles/unequal.c",
            "c/puzzles/unruly.c",
            "c/puzzles/untangle.c",
        ])
        .compile("puzzles");
        
    csbindgen::Builder::default()
        .input_bindgen_file("src/puzzles.rs")
        //.method_filter(|x| x.starts_with("puzzles_"))
        .rust_method_prefix("csbindgen_")
        .rust_file_header("use super::puzzles::*;")
        .rust_method_type_path("puzzles")
        .csharp_class_name("LibPuzzles")
        .csharp_namespace("CsBindgen")
        .csharp_dll_name("puzzles")
        .csharp_dll_name_if("UNITY_IOS && !UNITY_EDITOR", "__Internal")
        .csharp_entry_point_prefix("csbindgen_")
        .csharp_method_prefix("")
        .csharp_class_accessibility("public")
        //.csharp_c_long_convert("int")
        //.csharp_c_ulong_convert("uint")
        //.csharp_use_function_pointer(true)
        .csharp_use_function_pointer(false)
        .csharp_generate_const_filter(|_| true)
        .generate_to_file("src/puzzles_ffi.rs", "../dotnet-sandbox/puzzles_bindgen.cs")
        .unwrap();

    Ok(())
}

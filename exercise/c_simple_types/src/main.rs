// Silence some warnings so they don't distract from the exercise
#![allow(dead_code, unused_variables)]

use ding_machine::{ding, on_off, print_array, print_difference, print_distance};

fn main() {
    let coords: (f32, f32) = (6.3, 15.0);
    print_difference(coords.0, coords.1); // Uncomment and finish this line

    let coords_arr: [f32; 2] = [coords.0, coords.1];
    print_array(coords_arr); // and pass it in here (this line doesn't need to change)
                             //
    let series = [1, 1, 2, 3, 5, 8, 13];
    ding(series[series.len() - 1]);

    let mess = ([3, 2], 3.14, [(false, -3), (true, -100)], 5, "candy");
    on_off(mess.2[1].0);

    // 5.  What a mess -- functions in a binary! Let's get organized!
    //
    // - Make a library file (src/lib.rs)
    // - Move all the functions (except main) into the library
    // - Make all the functions public with `pub`
    // - Bring all the functions into scope using use statements. Remember, the name of the library
    //   is defined in Cargo.toml.  You'll need to know that to `use` it.
    //
    // `cargo run` should produce the same output, only now the code is more organized. 🎉

    // Challenge: Uncomment the line below, run the code, and examine the
    // output. Then go refactor the print_distance() function according to the
    // instructions in the comments inside that function.

    print_distance(coords);
}

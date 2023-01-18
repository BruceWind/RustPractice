#[macro_use]
extern crate litcrypt; //#litcrypt in there is used to hide string from decompiling.

use_litcrypt!();

fn main() {
    hideAnExposeStrEg();
}

// to give example for comparing between string was hidden and string exposed.
fn hideAnExposeStrEg() {
    // This string gives you an example for a case that a string could be get by decompling.
    // In this case, you can try run with string command to look into executable file, e.g:
    //        strings ./target/debug/RustPractice | grep exposure
    println!("exposure string.");
    // On the other hand, this string is another example for hiding from decompiling.
    //      also you can use the last command shown above to filter "hidden-string" with `grep hidden`.
    println!("{}", lc!("hidden-string."));
}

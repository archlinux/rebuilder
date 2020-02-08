use std::io::prelude::*;
use std::fs::File;
use tar::Builder;

// * generate pacman db and save to $tmpdir
// * specify --dbpath with test db's
// * run tests


#[test]
fn test_generate_db() {
    println!("create db");

    let pkg = "opencolorio";
    let rebuildorder = vec!["opencolorio", "krita", "krita-plugin-gmic", "openimageio", "openshadinglanguage", "blender"];

    let file = File::create("/tmp/core.db").unwrap();
    let mut builder = Builder::new(file);

    // Make up a version
    let ver = "1.0-1";
    // Create a "$pkgname-$ver" directory
    // Create a "desc" file in that directory with:
    // %FILENAME%
    // $pkgname-$pkgver.pkg.tar.zst
    //
    // %NAME%
    // $pkgname
    //
    // %BASE%
    // $pkgnae
}

use core::include;
use std::env;
use std::fs;
use std::path::Path;

use toml;

include!(concat!(env!("CARGO_MANIFSET_DIR"), "/source/core/config_structs.rs"));

fn main() {

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("config.rs");
    let mut out = String::new();

    let config_content = fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/config.toml")).unwrap();
    let dec_content: config = toml::from_str(&config_content[..]).unwrap();

    for fname in GeneralConfig::field_names().iter() {
        let meta = &dec_content.general.gen_meta_tuple(fname);
        let const_out = format!(
            "pub const {}_{} : {};\n",
            meta.0.to_uppercase().split("CONFIG").next().unwrap(),
            meta.1.to_uppercase(),
            meta.2,
            meta.3);
        out.push_str(&const_out);
    }
    for fname in PinConfig::field_names().iter() {
        let meta = &dec_content.general.gen_meta_tuple(fname);
        let const_out = format!(
            "pub const {}_{} : {};\n",
            meta.0.to_uppercase().split("CONFIG").next().unwrap(),
            meta.1.to_uppercase(),
            meta.2,
            meta.3);
        out.push_str(&const_out);
    }
    for fname in ControllerConfig::field_names().iter() {
        let meta = &dec_content.general.gen_meta_tuple(fname);
        let const_out = format!(
            "pub const {}_{} : {};\n",
            meta.0.to_uppercase().split("CONFIG").next().unwrap(),
            meta.1.to_uppercase(),
            meta.2,
            meta.3);
        out.push_str(&const_out);
    }
    for fname in DynamicConfig::field_names().iter() {
        let meta = &dec_content.general.gen_meta_tuple(fname);
        let const_out = format!(
            "pub const {}_{} : {};\n",
            meta.0.to_uppercase().split("CONFIG").next().unwrap(),
            meta.1.to_uppercase(),
            meta.2,
            meta.3);
        out.push_str(&const_out);
    }
    fs::write(&dest_path, out).unwrap();
    println!("cargo:rerun-if-changed=config.toml");
    println!("cargo:rerun-if-changed=build.rs");
}
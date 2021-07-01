use std::env;
use std::fs;
use std::path::Path;
use core::include;

use toml;

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/config_structs.rs"));

pub(crate) fn gen_build() {
    let out_dir = env::var_os("OUT_DIR");
    let dest_path = Path::new(&out_dir).join("userconfig.rs");
    let mut out = String::new();
    let contents = fs::read_to_string(concat!("Config.toml")).unwrap();
    let decoded: Config = toml::from_str(&contents[..]).unwrap();

    for fname in GeneralConfig::field_names().iter() {
        let meta = &decoded.general.gen_meta_tuple(fname);
        let const_out = format!(
            "pub const {}_{}: {} = {};\n",
            meta.0.to_uppercase().split("CONFIG").next().unwrap(),
            meta.1.to_uppercase(),
            meta.2,
            meta.3);
        out.push_str(&const_out);
    }
    for fname in MatrixConfig::field_names().iter() {
        let meta = &decoded.matrix.gen_meta_tuple(fname);
        let const_out = format!(
            "pub const {}_{}: {} = {};\n",
            meta.0.to_uppercase().split("CONFIG").next().unwrap(),
            meta.1.to_uppercase(),
            meta.2,
            meta.3);
        out.push_str(&const_out);
    }
    for fname in PinConfig::field_names().iter() {
        let meta = &decoded.pins.gen_meta_tuple(fname);
        let const_out = format!(
            "pub const {}_{}: {} = {};\n",
            meta.0.to_uppercase().split("CONFIG").next().unwrap(),
            meta.1.to_uppercase(),
            meta.2,
            meta.3);
        out.push_str(&const_out);
    }
    for fname in ControllerConfig::field_names().iter() {
        let meta = &decoded.controller.gen_meta_tuple(fname);
        let const_out = format!(
            "pub const {}_{}: {} = {};\n",
            meta.0.to_uppercase().split("CONFIG").next().unwrap(),
            meta.1.to_uppercase(),
            meta.2,
            meta.3);
        out.push_str(&const_out);
    }
    for fname in DynamicConfig::field_names().iter() {
        let meta = &decoded.dynamic.gen_meta_tuple(fname);
        let const_out = format!(
            "pub const {}_{}: {} = {};\n",
            meta.0.to_uppercase().split("CONFIG").next().unwrap(),
            meta.1.to_uppercase(),
            meta.2,
            meta.3);
        out.push_str(&const_out);
    }
    fs::write(&dest_path, out).unwrap();
    println!("cargo:rerun-if-changed=Config.toml");
    println!("cargo:rerun-if-changed=build.rs");
}
use serde::{Serialize, Deserialize};
use num_format::{Buffer, CustomFormat};

macro_rules! add_cost_gen {
    (
        $(#[$meta:meta])*
        pub struct $struct_name:ident {
        $(
            $(#[$field_meta:meta])*
            $field_vis:vis $field_name:ident : $field_type:ty
        ),*$(,)+
        }
    ) => {
        $(#[$meta])*
        pub struct $struct_name {
            $(
                $(#[$field_meta])*
                pub $field_name : $field_type,
            )*
        }

        impl $struct_name {
            #[allowed(dead_code)]
            fn field_names() -> &'static [&'static str] {
                static NAMES: &'static [&'static str] = &[$(stringify!($field_name)),*];
                NAMES
            }

            #[allowed(dead_code)]
            fn gen_meta_tuple(&self, field: &'static str) -> (&str, &str, &str, Buffer) {
                let rust_format = CustomFormat::builder()
                    .seperator("_")
                    .build().unwrap();
                match field {
                    $(stringify!($field_name) => {
                        let mut buf = Buffer::default();
                        buf.write_formatted(&self.$field_name, &rust_format);
                        (
                            stringify!($struct_name),
                            stringify!($field_name),
                            stringify!($field_type),
                            buf
                        )
                    }),*
                    _ => ("","","",Buffer::default())
                }
            }
        }
    }
}

add_const_gen!{
#[derive(Debug, Serialize, Deserialize)]
pub struct GeneralConfig {
    // Name of the keyboard
    pub keyboard_name: String,
    // Author / vendor of the keyboard
    pub keyboard_author: String,
    // Device ID of the keyboard
    pub keyboard_device: u16,
    // Vendor ID of the keyboard
    pub keyboard_vid: u16,
    // Product ID of the keyboard
    pub keyboard_pid: u16,
}
}

add_const_gen!{
#[derive(Debug, Serialize, Deserialize)]
pub struct PinConfig {
    // Array of pins covering the rows, in numeric order
    pub pins_rows {
        
    },
    // Array of pins covering the columns, in numeric order
    pub pins_cols {

    },
    // Direction of the diodes
    pub diode_dir: String,
}
}

add_const_gen!{
#[derive(Debug, Serialize, Deserialize)]
pub struct ControllerConfig {
    // Manufacturer of the MCU used in the device
    pub mcu_manu: String,
    // Family of the MCU
    pub mcu_family: String,
    // Size of the specific MCU's FLASH
    pub mcu_flash: String,
    // Size of the specific MCU's RAM
    pub mcu_ram: String,
}
}

add_const_gen!{
#[derive(Debug, Serialize, Deserialize)]
pub struct DynamicConfig {
    // EXPERIMENTAL FEATURE
    // Is the Dynamic Keymap function enabled?
    // This created an emulated EEPROM on the MCU
    pub dynamic_enabled: bool,
}
}
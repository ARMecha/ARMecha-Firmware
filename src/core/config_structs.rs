use serde::{Serialize, Deserialize};
use num_format::{Buffer, CustomFormat};
use core::fmt::Display;

struct IsInt<'__, T>(&'__ T);
impl<Int : ::num_format::ToFormattedStr> IsInt<'_, Int> {
    fn pretty_display (self: Self)
      -> impl 'static + ::core::fmt::Display
    {
        let ref rust_format =
            CustomFormat::builder()
                .separator("_")
                .build()
                .unwrap()
        ;
        let mut buf = Buffer::default();
        buf.write_formatted(self.0, rust_format);
        buf
    }
}

trait Fallback<'__, T> {
    fn pretty_display (self: Self)
      -> &'__ T
    ;
}
impl<'lt, T : ::core::fmt::Display> Fallback<'lt, T>
    for IsInt<'lt, T>
{
    fn pretty_display (self: Self)
      -> &'lt T
    {
        self.0
    }
}

macro_rules! add_const_gen {
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
            #[allow(dead_code)]
            fn field_names() -> &'static [&'static str] {
                static NAMES: &'static [&'static str] = &[$(stringify!($field_name)),*];
                NAMES
            }
            fn gen_meta_tuple (
                self: &'_ Self,
                field: &'static str,
            ) -> (&'_ str, &'_ str, &'_ str, Box<dyn '_ + Display>)
            {
                match field {
                $(
                    | stringify!($field_name) => (
                        stringify!($struct_name),
                        stringify!($field_name),
                        stringify!($field_type),
                        Box::new(IsInt(&self.$field_name).pretty_display()),
                    ),
                )*
                    | _ => ("", "", "", Box::new("")),
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
    pub struct MatrixConfig {
        // Amount of rows for the matrix
        pub matrix_rows: u8,
        // Amount of cols for the matrix
        pub matrix_cols: u8,
        // Directon of the diodes in the matrix
        pub matrix_diode_dir: String,
    }
    }

add_const_gen!{
#[derive(Debug, Serialize, Deserialize)]
pub struct PinConfig {
    // Pins of the rows in numerical order
    pub pins_rows: String,
    // Pins of the cols in numerical order
    pub pins_cols: String,
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
    pub dynamic_enabled: String,
}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    // General configuration items
    pub general: GeneralConfig,
    // Matrix configuration items
    pub matrix: MatrixConfig,
    // Pin configuration items
    pub pins: PinConfig,
    // Controller configuration items
    pub controller: ControllerConfig,
    // Dynamic Keymap configuration items
    pub dynamic: DynamicConfig,
}
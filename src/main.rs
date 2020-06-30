extern crate gphoto;

use gphoto::Camera;

use std::io::{self, Read};
use std::io::prelude::*;
use std::path::Path;


/// (See http://www.gphoto.org/doc/manual/ref-gphoto2-cli.html for info)
/// Here's the basic plan so far:
///     1) Get all available configurable settings using `--list-config`
///     2) Get possible config values for all config entries using `--get-config`
///         a) Previous two steps could also be done in one step with `--list-all-config`
///     3) Prompt user for settings for everything
///     4) Take exposure(s)
///     5) Allow user to export settings to config file for future use without entering each
///        setting value


fn main() {




    let mut context = gphoto::Context::new().unwrap();

    let mut camera = gphoto::Camera::autodetect(&mut context).unwrap();
    // let capture = camera.capture_image(&mut context).unwrap();
    // let mut file = gphoto::FileMedia::create(Path::new(&*capture.basename())).unwrap();
    // camera.download(&mut context, &capture, &mut file).unwrap();

    let abilities = camera.abilities();

    println!("      device type = {:?}", abilities.device_type());
    println!("            model = {:?}", abilities.model());
    println!("    driver status = {:?}", abilities.driver_status());
    println!("       port types = {:?}", abilities.port_types());
    println!("           speeds = {:?}", abilities.speeds());
    println!("camera operations = {:?}", abilities.camera_operations());
    println!("  file operations = {:?}", abilities.file_operations());
    println!("folder operations = {:?}", abilities.folder_operations());
    println!("       USB vendor = {:?}", abilities.usb_vendor());
    println!("      USB product = {:?}", abilities.usb_product());
    println!("        USB class = {:?}", abilities.usb_class());
    println!("     USB subclass = {:?}", abilities.usb_subclass());
    println!("     USB protocol = {:?}", abilities.usb_protocol());

    get_and_set_iso(camera);
}

fn get_and_set_iso(camera: gphoto::Camera) {
    println!("Enter desired ISO: ");
    // let iso: i32 = io::stdin() as i32;
    let mut str_buf = String::new();
    io::stdin().read_to_string(&mut str_buf).unwrap();
    let iso = str_buf.parse::<i16>().unwrap();
}

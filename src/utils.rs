use hidapi::HidApi;
fn list_devices() {
    println!("Printing all available hid devices:");

    match HidApi::new() {
        Ok(api) => {
            for device in api.device_list() {
                println!("{:04x}:{:04x}", device.vendor_id(), device.product_id());
                if let Ok(open_device) = device.open_device(&api) {
                    if let Ok(product_string) = open_device.get_product_string() {
                        println!("product: {:?}", product_string);
                    }
                    if let Ok(manufacture_string) = open_device.get_manufacturer_string() {
                        println!("manufacture: {:?}", manufacture_string);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}

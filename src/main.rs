mod utils;

const VID: u16 = 0x256c;
const PID: u16 = 0x006d;

fn main() {
    let api = hidapi::HidApi::new().expect("Failed to initialize hidapi");
    match api.open(VID, PID) {
        Ok(device) => {
            for i in 0..100 {
                let mut buffer = [0u8; 8];
                device
                    .read(&mut buffer)
                    .expect("Failed to read from device");
                println!("read data {i}: {:?}", buffer);
            }
        }
        Err(err) => println!(
            "Error while trying to open device {:04x}:{:04x} : {}",
            VID, PID, err
        ),
    }
}

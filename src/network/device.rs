use tun::AsyncDevice;

pub fn get_default_tun() -> AsyncDevice {
    let mut tun_config = tun::Configuration::default();
    
    tun_config.address("72.100.100.100");
    // tun_config.layer(tun::Layer::L2);
    tun_config.up();

    let device = tun::create(&tun_config);

    if device.is_err() {
        eprintln!("Failed to create device: {:#?}", device.err());
        std::process::exit(-1);
    }

    let device = device.unwrap();
    device.set_nonblock();
    AsyncDevice::new(device).unwrap()
}

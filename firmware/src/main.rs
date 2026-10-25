use std::{thread, time::Duration};

use anyhow::Result;
use embedded_hal::spi::MODE_3;
use embedded_graphics::{
    image::Image,
    image::ImageRawLE,
    pixelcolor::Rgb565,
    prelude::*,
};
use esp_idf_hal::{
    delay::Ets,
    gpio::*,
    peripherals::Peripherals,
    prelude::*,
    spi::*,
    task::block_on,
    units::FromValueType,
};
use display_interface_spi::SPIInterfaceNoCS;
use mipidsi::{Builder, Orientation};

fn main() -> Result<()> {
    esp_idf_sys::link_patches();

    block_on(async_main())
}

async fn async_main() -> Result<()> {
    let peripherals = Peripherals::take()?;
    let spi = peripherals.spi2;

    // Pins for display
    let rst = PinDriver::output(peripherals.pins.gpio3)?;
    let dc = PinDriver::output(peripherals.pins.gpio4)?;
    let mut backlight = PinDriver::output(peripherals.pins.gpio5)?;
    let sclk = peripherals.pins.gpio6;
    let sda = peripherals.pins.gpio7; // MOSI
    let sdi = peripherals.pins.gpio8; // MISO
    let cs = peripherals.pins.gpio10;

    let mut delay = Ets;

    // Configure async SPI
    let spi_driver = SpiDriver::new_async(
        spi,
        sclk,
        sda,
        Some(sdi),
        &SpiDriverConfig::new(),
    )?;

    let config = config::Config::new()
        .baudrate(26.MHz().into())
        .data_mode(MODE_3);

    let spi_device = SpiDeviceDriver::new(&spi_driver, Some(cs), &config)?;

    // Interface abstraction for mipidsi
    let di = SPIInterfaceNoCS::new(spi_device, dc);

    // Init display
    let mut display = Builder::st7789(di)
        .with_display_size(240, 240)
        .with_orientation(Orientation::Portrait(false))
        .init(&mut delay, Some(rst))?;

    backlight.set_high()?;

    let raw_image_data = ImageRawLE::new(include_bytes!("../examples/assets/ferris.raw"), 86);
    let ferris = Image::new(&raw_image_data, Point::new(0, 0));

    display.clear(Rgb565::BLACK)?;
    ferris.draw(&mut display)?;

    println!("Image drawn!");

    loop {
        thread::sleep(Duration::from_secs(1));
    }
}

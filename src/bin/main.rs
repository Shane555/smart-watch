#![no_std]
#![no_main]

use core::cell::RefCell;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::blocking_mutex::Mutex;
use esp_backtrace as _;
use static_cell::StaticCell;
use esp_hal::{
    clock::CpuClock,
    Async,
    dma::{DmaRxBuf, DmaTxBuf},
    dma_buffers,
    delay::Delay,
    gpio,
    spi::{
        master::{Config, Spi},
        Mode,
    },
    time::Rate,
    timer::timg::TimerGroup,
    peripherals::SPI2,
};
use log::info;
use mipidsi::{Builder, options::Orientation, models::ST7789, Display as MipiDisplay};
use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
    mono_font::{ascii::FONT_9X15, MonoTextStyle},
    text::Text,
};
use display_interface_spi::SPIInterface;

type DisplayType = MipiDisplay<
    SPIInterface<Spi<'static, Async>, gpio::AnyPin>,
    ST7789,
    gpio::AnyPin,
>;
static DISPLAY_CELL: StaticCell<Mutex<NoopRawMutex, RefCell<Option<DisplayType>>>> = StaticCell::new();

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    esp_println::logger::init_logger_from_env();
    // boiler plate intialiser for peripherals/system
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    //boiler plate intialiser for embassy
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_hal_embassy::init(timg0.timer0);
    info!("Embassy initialized!");

    let sclk = peripherals.GPIO4;
    let miso = gpio::NoPin;
    let mosi = peripherals.GPIO6;
    let cs = gpio::NoPin;
    let dma_channel = peripherals.DMA_CH0;

    let io = gpio::Io::new(peripherals.IO_MUX);
    // let dc = io.
    // let rst = io.gpio3.into_push_pull_output();
    let (rx_buffer, rx_descriptors, tx_buffer, tx_descriptors) = dma_buffers!(32000);
    let dma_rx_buf = DmaRxBuf::new(rx_descriptors, rx_buffer).unwrap();
    let dma_tx_buf = DmaTxBuf::new(tx_descriptors, tx_buffer).unwrap();

    let mut spi = Spi::new(
        peripherals.SPI2,
        Config::default()
            .with_frequency(Rate::from_khz(100))
            .with_mode(Mode::_0),
    )
    .unwrap()
    .with_sck(sclk)
    .with_mosi(mosi)
    .with_miso(miso)
    .with_cs(cs)
    .with_dma(dma_channel)
    .with_buffers(dma_rx_buf, dma_tx_buf)
    .into_async();

    let mut delay = Delay::new();
    let interface = SPIInterface::new(spi, dc);
    let mut display = Builder::new(ST7789, &mut interface)
        .with_display_size(240, 240)
        .with_orientation(mipidsi::options::Orientation::Portrait)
        .init(&mut delay, Some(rst))
        .unwrap();

    // TODO: Spawn some tasks
    let _ = spawner;

    let style = MonoTextStyle::new(&FONT_9X15, Rgb565::GREEN);
    Text::new("Hello, DMA LCD!", Point::new(20, 40), style)
        .draw(&mut display)
        .unwrap();

    let display_mutex = Mutex::new(RefCell::new(Some(display)));
    let display_ref = DISPLAY_CELL.init(display_mutex);
    spawner.spawn(blink_text(display_ref)).unwrap();

    let send_buffer = [0, 1, 2, 3, 4, 5, 6, 7];
    loop {
        let mut buffer = [0; 8];
        esp_println::println!("Sending bytes");
        embedded_hal_async::spi::SpiBus::transfer(&mut spi, &mut buffer, &send_buffer)
            .await
            .unwrap();
        esp_println::println!("Bytes received: {:?}", buffer);
        Timer::after(Duration::from_millis(5_000)).await;
    }
    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-beta.0/examples/src/bin
}

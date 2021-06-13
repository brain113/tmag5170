#![no_std]
#![no_main]

use core::convert::TryInto;

use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use cortex_m::asm;
use cortex_m_rt::entry;
use stm32f3xx_hal::{pac, prelude::*, spi::Spi};
use tmag5170::{self, alert_config, device_config, sensor_config, system_config, ExtError};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(48.MHz()).freeze(&mut flash.acr);
    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);

    rtt_init_print!(NoBlockSkip, 4096);
    rprintln!("pre init");

    let mut cs = gpioa
        .pa10
        .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);
    cs.set_high().unwrap();

    let alert = gpioa
        .pa8
        .into_pull_up_input(&mut gpioa.moder, &mut gpioa.pupdr);

    let sck = gpioa
        .pa5
        .into_af5_push_pull(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrl);
    let miso = gpioa
        .pa6
        .into_af5_push_pull(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrl);
    let mosi = gpioa
        .pa7
        .into_af5_push_pull(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrl);

    let spi = Spi::spi1(
        dp.SPI1,
        (sck, miso, mosi),
        tmag5170::MODE,
        1.MHz().try_into().unwrap(),
        clocks,
        &mut rcc.apb2,
    );

    let mut tmag5170 = tmag5170::Tmag5170::new(spi, cs);

    let config = sensor_config::SensorConfig::new()
        .set_angle_en(sensor_config::AngleEn::Xy)
        .set_sleep_time(sensor_config::SleepTime::Ms500)
        .set_mag_ch_en(sensor_config::MagChEn::Xyz)
        .set_z_range(sensor_config::Range::A1_100mT_A2_300mT)
        .set_y_range(sensor_config::Range::A1_100mT_A2_300mT)
        .set_x_range(sensor_config::Range::A1_100mT_A2_300mT);
    let _ = tmag5170.apply_sensor_config(config);

    let config = system_config::SystemConfig::new()
        .set_diag_sel(system_config::DiagSel::AllDataInSeq)
        .set_trigger_mode(system_config::TriggerMode::Spi)
        .set_data_type(system_config::DataType::AM)
        .set_diag_sel(system_config::DiagSel::AllDataInSeq)
        .set_diag_en(false)
        .set_t_z_limit_check(false)
        .set_t_y_limit_check(false)
        .set_t_x_limit_check(false);
    let _ = tmag5170.apply_system_config(config);

    let config = alert_config::AlertConfig::new()
        .set_alert_latch(alert_config::AlertLatch::NotLatched)
        .set_alert_mode(alert_config::AlertMode::Interrupt)
        .set_rslt_alrt_enable(true);
    let _ = tmag5170.apply_alert_config(config);

    let config = device_config::DeviceConfig::new()
        .set_conv_avg(device_config::ConvAvg::Avg32x)
        .set_mag_tempco(device_config::MagTempco::NdBFe)
        // WakeupAndSleep doesn't work in pre-release device
        .set_operating_mode(device_config::OperatingMode::Active)
        .set_t_en(true)
        .set_t_rate(device_config::TRate::OncePerConvSet)
        .set_t_limit_check_en(false)
        .set_t_comp_en(true);
    let _ = tmag5170.apply_device_config(config);

    rprintln!("post init");

    loop {
        // Delay
        for _i in 0..50_000 {
            asm::nop();
        }

        // wait for alert event
        while alert.is_high().unwrap() {}

        let res = tmag5170.read_am();

        match res {
            Err(ext) => match ext {
                ExtError::CrcError => rprintln!("Crc Error"),
                ExtError::E(_e) => rprintln!("SPI Error"),
            },
            Ok((a, m)) => rprintln!("Angle {:3} deg, magnitude {}", a / 8, m),
        }
    }
}

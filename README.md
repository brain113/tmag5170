# Platform agnostic Rust driver for the TMAG5170-Q1 3D hall sensor

The TMAG5170-Q1 is a 3-axis linear Hall effect sensor designed for automotive and industrial
applications. This device integrates 3 independent Hall sensors in X, Y, and Z axes. A precision analog
signal-chain along with integrated 12-bit ADC digitizes the measured analog magnetic field values.
The SPI interface can be used by an external microcontroller to configure the device, start a conversion,
or to read back the device register data. On-chip integrated temperature sensor data is available for
multiple system functions such as safety check and temperature compensation for a given magnetic field
measurement.


## The device
https://www.ti.com/product/TMAG5170-Q1

## Usage

Include this driver to your Cargo.toml
```
[dependencies]
tmag5170 = "<version>"
```
Use embedded-hal implementation to get SPI and a GPIO OutputPin for the chip select, then create the magnetometer handle. There is the basic usage of the driver.
```rust
use tmag5170::{self, ExtError, device_config, sensor_config, system_config, alert_config};

...
  let mut tmag5170 = tmag5170::Tmag5170::new(spi, cs);

  let config = sensor_config::SensorConfig::new()
      .set_angle_en(sensor_config::AngleEn::XY)
      .set_sleep_time(sensor_config::SleepTime::Ms500)
      .set_mag_ch_en(sensor_config::MagChEn::XYZ)
      .set_z_range(sensor_config::Range::A1_100mT_A2_300mT)
      .set_y_range(sensor_config::Range::A1_100mT_A2_300mT)
      .set_x_range(sensor_config::Range::A1_100mT_A2_300mT);
  let _ = tmag5170.apply_sensor_config(config);


  let config = system_config::SystemConfig::new()
      .set_diag_sel(system_config::DiagSel::AllDataInSeq)
      .set_trigger_mode(system_config::TriggerMode::SPI)
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

  let res = tmag5170.read_am();

  match res {
      Err(ext) => 
          match ext {
              ExtError::CrcError => rprintln!("Crc Error"),
              ExtError::E(_e) => rprintln!("SPI Error"),
          },
          Ok((a,m)) => rprintln!("Angle {:3} deg, magnitude {}", a / 8, m),
  }

```

## Dependencies
To build embedded programs using this template you'll need:

- Rust 1.31, 1.30-beta, nightly-2018-09-13 or a newer toolchain. e.g. `rustup
  default beta`

# License

This template is licensed under either of

- Apache License, Version 2.0 (http://www.apache.org/licenses/LICENSE-2.0)

- MIT license (http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Code of Conduct

Contribution to this crate is organized under the terms of the [Rust Code of
Conduct][CoC], the maintainer of this crate, the [Cortex-M team][team], promises
to intervene to uphold that code of conduct.

[CoC]: https://www.rust-lang.org/policies/code-of-conduct
[team]: https://github.com/rust-embedded/wg#the-cortex-m-team

## Limitations

At the moment the CRC4 table is stored in every tmag5170 instance.
This should be not a problem with the only tmag5170 in the systemm
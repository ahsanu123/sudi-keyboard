#![no_std]
#![no_main]

#[cfg(not(feature = "defmt"))]
use panic_halt as _;

#[cfg(feature = "defmt")]
use {defmt_rtt as _, panic_probe as _};

use embassy_executor::Spawner;
use embassy_futures::join::join;
use embassy_futures::select::select;
use embassy_nrf::gpio::{Level, Output, OutputDrive};
use embassy_time::Timer;
use sudi_firmware::info;
use trouble_host::prelude::*;

const CONNECTION_MAX: usize = 1;
const L2CAP_CHANNEL_MAX: usize = 2;

#[gatt_server]
struct Server {
    battery_service: BatteryService,
}

#[gatt_service(uuid = service::BATTERY)]
struct BatteryService {
    #[descriptor(uuid = descriptors::VALID_RANGE, read, value = [0, 100])]
    #[descriptor(uuid = descriptors::MEASUREMENT_DESCRIPTION, name = "hello world", read, value ="Battery Level")]
    #[descriptor(uuid = descriptors::BATTERY_LEVEL, read, notify, value =10)]
    level: u8,

    #[characteristic(uuid = "408813df-5dd4-1f87-ec11-cdb001100000", write, read, notify)]
    status: bool,
}

pub async fn run<C>(controller: C)
where
    C: Controller,
{
    // Using a fixed "random" address can be useful for testing. In real scenarios, one would
    // use e.g. the MAC 6 byte array as the address (how to get that varies by the platform).
    let address: Address = Address::random([0xff, 0x8f, 0x1a, 0x05, 0xe4, 0xff]);
    info!("Our address = {:?}", address);

    let mut resources: HostResources<PacketPool, CONNECTION_MAX, L2CAP_CHANNEL_MAX> =
        HostResources::new();

    let stack = trouble_host::new(controller, &mut resources).set_random_address(address);
    let Host {
        mut peripheral,
        runner,
        ..
    } = stack.build();

    info!("Starting advertising and GATT service");

    let peripheral_config = PeripheralConfig {
        name: "AHAH Trouble",
        appearance: &appearance::power_device::GENERIC_POWER_DEVICE,
    };
    let server = Server::new_with_config(GapConfig::Peripheral(peripheral_config)).unwarp();

    // let _ = join(ble_task(runner))
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());
    let mut led = Output::new(p.P0_13, Level::Low, OutputDrive::Standard);

    loop {
        info!("Hello, World!");
        led.set_high();
        Timer::after_millis(500).await;
        led.set_low();
        Timer::after_millis(500).await;
    }
}

// async fn ble_task<C: Controller, P: PacketPool>(mut runner: Runner<'_, C, P>) {
//     loop {
//         if let Err(e) = runner.run().await {
//             let e = defmt::Debug2Format(&e);
//             panic!("[ble_task] error {:?}", e);
//         }
//     }
// }

// async fn gatt_events_task<P: PacketPool>(
//     server: &Server<'_>,
//     conn: &GattConnection<'_, '_, P>,
// ) -> Result<(), Error> {
//     let level = server.battery_service.level;
//     let reason = loop {
//         match conn.next().await {
//             GattConnectionEvent::Disconnected { reason } => break reason,
//             GattConnectionEvent::Gatt { event: Err(e) } => {
//                 warn!("[gatt] error processing event: {:?}", e)
//             }
//             GattConnectionEvent::Gatt { event: Ok(event) } => {
//                 match &event {
//                     GattEvent::Read(event) => {
//                         if event.handle() == level.handle {
//                             let value = server.get(&level);
//                             info!("[gatt] Read Event to Level Characteristic: {:?}", value);
//                         }
//                     }
//                     GattEvent::Write(event) => {
//                         if event.handle() == level.handle {
//                             info!(
//                                 "[gatt] Write Event to Level Characteristic: {:?}",
//                                 event.data()
//                             );
//                         }
//                     }
//                 };
//                 // This step is also performed at drop(), but writing it explicitly is necessary
//                 // in order to ensure reply is sent.
//                 match event.accept() {
//                     Ok(reply) => reply.send().await,
//                     Err(e) => warn!("[gatt] error sending response: {:?}", e),
//                 };
//             }
//             _ => {} // ignore other Gatt Connection Events
//         }
//     };
//     info!("[gatt] disconnected: {:?}", reason);
//     Ok(())
// }

// async fn advertise<'values, 'server, C: Controller>(
//     name: &'values str,
//     peripheral: &mut Peripheral<'values, C, DefaultPacketPool>,
//     server: &'server Server<'values>,
// ) -> Result<GattConnection<'values, 'server, DefaultPacketPool>, BleHostError<C::Error>> {
//     let mut advertiser_data = [0; 31];
//     let len = AdStructure::encode_slice(
//         &[
//             AdStructure::Flags(LE_GENERAL_DISCOVERABLE | BR_EDR_NOT_SUPPORTED),
//             AdStructure::ServiceUuids16(&[[0x0f, 0x18]]),
//             AdStructure::CompleteLocalName(name.as_bytes()),
//         ],
//         &mut advertiser_data[..],
//     )?;
// }

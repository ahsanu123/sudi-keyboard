pub mod ble_bas_peripheral_sec {
    use embassy_futures::join::join;
    use embassy_futures::select::select;
    use embassy_time::Timer;
    use rand_core::{CryptoRng, RngCore};
    use trouble_host::prelude::*;
    /// Max number of connections
    const CONNECTIONS_MAX: usize = 1;
    /// Max number of L2CAP channels.
    const L2CAP_CHANNELS_MAX: usize = 2;
    const _ATTRIBUTE_TABLE_SIZE: usize = trouble_host::gap::GAP_SERVICE_ATTRIBUTE_COUNT
        + BatteryService::ATTRIBUTE_COUNT;
    const _: () = {
        if !(_ATTRIBUTE_TABLE_SIZE
            >= trouble_host::gap::GAP_SERVICE_ATTRIBUTE_COUNT
                + BatteryService::ATTRIBUTE_COUNT)
        {
            {
                ::core::panicking::panic_fmt(
                    format_args!(
                        "Specified attribute table size is insufficient. Please increase attribute_table_size or remove the argument entirely to allow automatic sizing of the attribute table.",
                    ),
                );
            }
        }
    };
    const _CCCD_TABLE_SIZE: usize = 0 + BatteryService::CCCD_COUNT;
    const _CONNECTIONS_MAX: usize = 1;
    struct Server<'values> {
        pub server: trouble_host::prelude::AttributeServer<
            'values,
            embassy_sync::blocking_mutex::raw::NoopRawMutex,
            trouble_host::prelude::DefaultPacketPool,
            _ATTRIBUTE_TABLE_SIZE,
            _CCCD_TABLE_SIZE,
            _CONNECTIONS_MAX,
        >,
        battery_service: BatteryService,
    }
    impl<'values> Server<'values> {
        /// Create a new Gatt Server instance.
        ///
        /// Requires you to add your own GAP Service.  Use `new_default(name)` or `new_with_config(name, gap_config)` if you want to add a GAP Service.
        fn new(
            mut table: trouble_host::attribute::AttributeTable<
                'values,
                embassy_sync::blocking_mutex::raw::NoopRawMutex,
                _ATTRIBUTE_TABLE_SIZE,
            >,
        ) -> Self {
            let battery_service = BatteryService::new(&mut table);
            Self {
                server: trouble_host::prelude::AttributeServer::new(table),
                battery_service,
            }
        }
        /// Create a new Gatt Server instance.
        ///
        /// This function will add a Generic GAP Service with the given name.
        /// The maximum length which the name can be is 22 bytes (limited by the size of the advertising packet).
        /// If a name longer than this is passed, Err() is returned.
        fn new_default(name: &'values str) -> Result<Self, &'static str> {
            let mut table: trouble_host::attribute::AttributeTable<
                '_,
                embassy_sync::blocking_mutex::raw::NoopRawMutex,
                _ATTRIBUTE_TABLE_SIZE,
            > = trouble_host::attribute::AttributeTable::new();
            trouble_host::gap::GapConfig::default(name).build(&mut table)?;
            let battery_service = BatteryService::new(&mut table);
            Ok(Self {
                server: trouble_host::prelude::AttributeServer::new(table),
                battery_service,
            })
        }
        /// Create a new Gatt Server instance.
        ///
        /// This function will add a GAP Service.
        /// The maximum length which the device name can be is 22 bytes (limited by the size of the advertising packet).
        /// If a name longer than this is passed, Err() is returned.
        fn new_with_config(
            gap: trouble_host::gap::GapConfig<'values>,
        ) -> Result<Self, &'static str> {
            let mut table: trouble_host::attribute::AttributeTable<
                '_,
                embassy_sync::blocking_mutex::raw::NoopRawMutex,
                _ATTRIBUTE_TABLE_SIZE,
            > = trouble_host::attribute::AttributeTable::new();
            gap.build(&mut table)?;
            let battery_service = BatteryService::new(&mut table);
            Ok(Self {
                server: trouble_host::prelude::AttributeServer::new(table),
                battery_service,
            })
        }
        fn get<T: trouble_host::attribute::AttributeHandle<Value = V>, V: FromGatt>(
            &self,
            attribute_handle: &T,
        ) -> Result<T::Value, trouble_host::Error> {
            self.server.table().get(attribute_handle)
        }
        fn set<T: trouble_host::attribute::AttributeHandle>(
            &self,
            attribute_handle: &T,
            input: &T::Value,
        ) -> Result<(), trouble_host::Error> {
            self.server.table().set(attribute_handle, input)
        }
        fn get_cccd_table(
            &self,
            connection: &trouble_host::connection::Connection<
                '_,
                trouble_host::prelude::DefaultPacketPool,
            >,
        ) -> Option<trouble_host::prelude::CccdTable<_CCCD_TABLE_SIZE>> {
            self.server.get_cccd_table(connection)
        }
        fn set_cccd_table(
            &self,
            connection: &trouble_host::connection::Connection<
                '_,
                trouble_host::prelude::DefaultPacketPool,
            >,
            table: trouble_host::prelude::CccdTable<_CCCD_TABLE_SIZE>,
        ) {
            self.server.set_cccd_table(connection, table);
        }
    }
    impl<'values> core::ops::Deref for Server<'values> {
        type Target = trouble_host::prelude::AttributeServer<
            'values,
            embassy_sync::blocking_mutex::raw::NoopRawMutex,
            trouble_host::prelude::DefaultPacketPool,
            _ATTRIBUTE_TABLE_SIZE,
            _CCCD_TABLE_SIZE,
            _CONNECTIONS_MAX,
        >;
        fn deref(&self) -> &Self::Target {
            &self.server
        }
    }
    struct BatteryService {
        level_hello_descriptor: trouble_host::attribute::Descriptor<&'static [u8]>,
        /// Battery Level
        level: trouble_host::attribute::Characteristic<u8>,
        status: trouble_host::attribute::Characteristic<bool>,
        handle: u16,
    }
    #[allow(unused)]
    impl BatteryService {
        const ATTRIBUTE_COUNT: usize = 9usize;
        const CCCD_COUNT: usize = 2usize;
        fn new<M, const MAX_ATTRIBUTES: usize>(
            table: &mut trouble_host::attribute::AttributeTable<'_, M, MAX_ATTRIBUTES>,
        ) -> Self
        where
            M: embassy_sync::blocking_mutex::raw::RawMutex,
        {
            let mut service = table
                .add_service(
                    trouble_host::attribute::Service::new({
                        let uuid: trouble_host::types::uuid::Uuid = service::BATTERY
                            .into();
                        uuid
                    }),
                );
            let (level, level_hello_descriptor) = {
                static LEVEL: static_cell::StaticCell<
                    [u8; <u8 as trouble_host::types::gatt_traits::AsGatt>::MAX_SIZE],
                > = static_cell::StaticCell::new();
                let store = LEVEL
                    .init(
                        [0; <u8 as trouble_host::types::gatt_traits::AsGatt>::MAX_SIZE],
                    );
                let mut builder = service
                    .add_characteristic(
                        {
                            let uuid: trouble_host::types::uuid::Uuid = characteristic::BATTERY_LEVEL
                                .into();
                            uuid
                        },
                        &[
                            trouble_host::attribute::CharacteristicProp::Read,
                            trouble_host::attribute::CharacteristicProp::Notify,
                        ],
                        10,
                        store,
                    );
                {
                    let value = [0, 100];
                    static DESC_0_LEVEL: static_cell::StaticCell<
                        [u8; [0, 100].len() as usize],
                    > = static_cell::StaticCell::new();
                    let store = DESC_0_LEVEL.init([0; [0, 100].len() as usize]);
                    let value = trouble_host::types::gatt_traits::AsGatt::as_gatt(
                        &value,
                    );
                    store[..value.len()].copy_from_slice(value);
                    builder
                        .add_descriptor::<
                            &[u8],
                            _,
                        >(
                            {
                                let uuid: trouble_host::types::uuid::Uuid = descriptors::VALID_RANGE
                                    .into();
                                uuid
                            },
                            &[trouble_host::attribute::CharacteristicProp::Read],
                            store,
                        )
                };
                let level_hello_descriptor = {
                    let value = "Battery Level";
                    static DESC_1_LEVEL: static_cell::StaticCell<
                        [u8; "Battery Level".len() as usize],
                    > = static_cell::StaticCell::new();
                    let store = DESC_1_LEVEL.init([0; "Battery Level".len() as usize]);
                    let value = trouble_host::types::gatt_traits::AsGatt::as_gatt(
                        &value,
                    );
                    store[..value.len()].copy_from_slice(value);
                    builder
                        .add_descriptor::<
                            &[u8],
                            _,
                        >(
                            {
                                let uuid: trouble_host::types::uuid::Uuid = descriptors::MEASUREMENT_DESCRIPTION
                                    .into();
                                uuid
                            },
                            &[trouble_host::attribute::CharacteristicProp::Read],
                            store,
                        )
                };
                (builder.build(), level_hello_descriptor)
            };
            let (status,) = {
                static STATUS: static_cell::StaticCell<
                    [u8; <bool as trouble_host::types::gatt_traits::AsGatt>::MAX_SIZE],
                > = static_cell::StaticCell::new();
                let store = STATUS
                    .init(
                        [0; <bool as trouble_host::types::gatt_traits::AsGatt>::MAX_SIZE],
                    );
                let mut builder = service
                    .add_characteristic(
                        ::trouble_host::types::uuid::Uuid::new_long([
                            0u8,
                            0u8,
                            16u8,
                            1u8,
                            176u8,
                            205u8,
                            17u8,
                            236u8,
                            135u8,
                            31u8,
                            212u8,
                            93u8,
                            223u8,
                            19u8,
                            136u8,
                            64u8,
                        ]),
                        &[
                            trouble_host::attribute::CharacteristicProp::Read,
                            trouble_host::attribute::CharacteristicProp::Write,
                            trouble_host::attribute::CharacteristicProp::Notify,
                        ],
                        <bool>::default(),
                        store,
                    );
                (builder.build(),)
            };
            Self {
                handle: service.build(),
                level_hello_descriptor,
                level,
                status,
            }
        }
    }
    /// Run the BLE stack.
    pub async fn run<C, RNG>(controller: C, random_generator: &mut RNG)
    where
        C: Controller,
        RNG: RngCore + CryptoRng,
    {
        let address: Address = Address::random([0xff, 0x8f, 0x1a, 0x05, 0xe4, 0xff]);
        {
            let _ = (&address);
        };
        let mut resources: HostResources<
            DefaultPacketPool,
            CONNECTIONS_MAX,
            L2CAP_CHANNELS_MAX,
        > = HostResources::new();
        let stack = trouble_host::new(controller, &mut resources)
            .set_random_address(address)
            .set_random_generator_seed(random_generator);
        let Host { mut peripheral, runner, .. } = stack.build();
        {
            let _ = ();
        };
        let server = Server::new_with_config(
                GapConfig::Peripheral(PeripheralConfig {
                    name: "TrouBLE",
                    appearance: &appearance::power_device::GENERIC_POWER_DEVICE,
                }),
            )
            .unwrap();
        let _ = join(
                ble_task(runner),
                async {
                    loop {
                        match advertise("Trouble Example", &mut peripheral, &server)
                            .await
                        {
                            Ok(conn) => {
                                let a = gatt_events_task(&server, &conn);
                                let b = custom_task(&server, &conn, &stack);
                                select(a, b).await;
                            }
                            Err(e) => {
                                {
                                    {
                                        ::core::panicking::panic_fmt(
                                            format_args!("[adv] error: {0:?}", e),
                                        );
                                    };
                                };
                            }
                        }
                    }
                },
            )
            .await;
    }
    /// This is a background task that is required to run forever alongside any other BLE tasks.
    ///
    /// ## Alternative
    ///
    /// If you didn't require this to be generic for your application, you could statically spawn this with i.e.
    ///
    /// ```rust,ignore
    ///
    /// #[embassy_executor::task]
    /// async fn ble_task(mut runner: Runner<'static, SoftdeviceController<'static>>) {
    ///     runner.run().await;
    /// }
    ///
    /// spawner.must_spawn(ble_task(runner));
    /// ```
    async fn ble_task<C: Controller, P: PacketPool>(mut runner: Runner<'_, C, P>) {
        loop {
            if let Err(e) = runner.run().await {
                {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!("[ble_task] error: {0:?}", e),
                        );
                    };
                };
            }
        }
    }
    /// Stream Events until the connection closes.
    ///
    /// This function will handle the GATT events and process them.
    /// This is how we interact with read and write requests.
    async fn gatt_events_task(
        server: &Server<'_>,
        conn: &GattConnection<'_, '_, DefaultPacketPool>,
    ) -> Result<(), Error> {
        let level = server.battery_service.level;
        let reason = loop {
            match conn.next().await {
                GattConnectionEvent::Disconnected { reason } => break reason,
                GattConnectionEvent::Gatt { event } => {
                    let result = match &event {
                        GattEvent::Read(event) => {
                            if event.handle() == level.handle {
                                let value = server.get(&level);
                                {
                                    let _ = (&value);
                                };
                            }
                            None
                        }
                        GattEvent::Write(event) => {
                            if event.handle() == level.handle {
                                {
                                    let _ = (&event.data());
                                };
                            }
                            None
                        }
                        _ => None,
                    };
                    let reply_result = if let Some(code) = result {
                        event.reject(code)
                    } else {
                        event.accept()
                    };
                    match reply_result {
                        Ok(reply) => reply.send().await,
                        Err(e) => {
                            let _ = (&e);
                        }
                    }
                }
                _ => {}
            }
        };
        {
            let _ = (&reason);
        };
        Ok(())
    }
    /// Create an advertiser to use to connect to a BLE Central, and wait for it to connect.
    async fn advertise<'values, 'server, C: Controller>(
        name: &'values str,
        peripheral: &mut Peripheral<'values, C, DefaultPacketPool>,
        server: &'server Server<'values>,
    ) -> Result<
        GattConnection<'values, 'server, DefaultPacketPool>,
        BleHostError<C::Error>,
    > {
        let mut advertiser_data = [0; 31];
        let len = AdStructure::encode_slice(
            &[
                AdStructure::Flags(LE_GENERAL_DISCOVERABLE | BR_EDR_NOT_SUPPORTED),
                AdStructure::ServiceUuids16(&[[0x0f, 0x18]]),
                AdStructure::CompleteLocalName(name.as_bytes()),
            ],
            &mut advertiser_data[..],
        )?;
        let advertiser = peripheral
            .advertise(
                &Default::default(),
                Advertisement::ConnectableScannableUndirected {
                    adv_data: &advertiser_data[..len],
                    scan_data: &[],
                },
            )
            .await?;
        {
            let _ = ();
        };
        let conn = advertiser.accept().await?.with_attribute_server(server)?;
        {
            let _ = ();
        };
        Ok(conn)
    }
    /// Example task to use the BLE notifier interface.
    /// This task will notify the connected central of a counter value every 2 seconds.
    /// It will also read the RSSI value every 2 seconds.
    /// and will stop when the connection is closed by the central or an error occurs.
    async fn custom_task<C: Controller, P: PacketPool>(
        server: &Server<'_>,
        conn: &GattConnection<'_, '_, P>,
        stack: &Stack<'_, C, P>,
    ) {
        let mut tick: u8 = 0;
        let level = server.battery_service.level;
        loop {
            tick = tick.wrapping_add(1);
            {
                let _ = (&tick);
            };
            if level.notify(conn, &tick).await.is_err() {
                {
                    let _ = ();
                };
                break;
            }
            if let Ok(rssi) = conn.raw().rssi(stack).await {
                {
                    let _ = (&rssi);
                };
            } else {
                {
                    let _ = ();
                };
                break;
            };
            Timer::after_secs(2).await;
        }
    }
}

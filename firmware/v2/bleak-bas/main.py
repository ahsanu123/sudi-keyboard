import asyncio

from bleak import BleakClient

ADDRESS = "FF:E4:05:1A:8F:FF"
BATTERY_CHAR_UUID = "00002a19-0000-1000-8000-00805f9b34fb"


def callback(sender, data: bytearray):
    print(f"{sender}: {int(data[0])}%")


async def main():
    async with BleakClient(ADDRESS) as client:
        await client.start_notify(BATTERY_CHAR_UUID, callback)
        print("subscribed to battery level notification")
        await asyncio.sleep(100)


asyncio.run(main())

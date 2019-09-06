# Embedded Rust - Bluetooth

## notes
https://docs.rust-embedded.org/discovery/13-serial-over-bluetooth/index.html

bluetooth ble resource:
https://www.novelbits.io/

### connecting the discovery board, with debugger

```console
cd /tmp
openocd -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg


cd /tmp
touch itm.txt
itmdump -F -f itm.txt
```


### connecting to the serial bus
use pins PA10 for RDR (read), and PA9 for DTR (transfer)
connect gnd to gnd

```console
dmesg | grep -i tty
# look for:
# FTDI USB Serial Device converter now attached to tty<something>

# Launch minicom after finding the device file:
minicom -D /dev/ttyUSB0 -b 115200
```

### for bluetooth:
use pins PA10 for RDR (read), and PA9 for DTR (transfer)
connect gnd and voltage

Paring bluetooth on Linux:
```console
bluetoothctl
power on
scan on
agent on
devices
# should see something like:
# Device 00:28:E4:40:00:06 HC-05

sudo rfcomm bind 0 00:18:E4:40:00:06

# Launch minicom after pairing and mounting bluetooth
minicom -D /dev/rfcomm0
# # OR try this:
# minicom -D /dev/rfcomm0 -b 115200

# release/destroy the rfcomm0 file:
sudo rfcomm release 0
```



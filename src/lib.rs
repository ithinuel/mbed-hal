//! PinNames are enums that contain a NC = (int) 0xFFFF_FFFF.
//! We can safely assume that PinName is equivalent to a i32;
//! PinMode might be smaller than a i32. Though, unless used in a structure the aapcs load it to a
//! 32bit register anyway.

type PortName = i32;
type PinName = i32;
type PinDirection = i32;
type PinMode = i32;

#[repr(C)]
struct PinMap {
    pin: PinName,
    peripheral: i32,
    function: i32,
}

extern "C" {
    fn pin_function(pin: PinName, function: i32);
    fn pin_mode(pin: u32, mode: i32);
    fn pinmap_peripheral(pin: PinName, map: *const PinMap) -> u32;
    fn pinmap_function(pin: PinName, map: *const PinMap) -> u32;
    fn pinmap_merge(a: u32, b: u32) -> u32;
    fn pinmap_pinout(pin: PinName, map: *const PinMap);
    fn pinmap_find_peripheral(pin: PinName, map: *const PinMap) -> u32;
    fn pinmap_find_function(pin: PinName, map: *const PinMap) -> u32;
}

extern "C" {
    fn port_pin(port: PortName, pin: i32) -> PinName;
    fn port_init(obj: *mut port_t, port: PortName, mask: i32, dir: PinDirection);
    fn port_mode(obj: *mut port_t, mode: PinMode);
    fn port_dir(obj: *mut port_t, dir: PinDirection);
    fn port_write(obj: *mut port_t, value: i32);
    fn port_read(obj: *mut port_t) -> i32;
}

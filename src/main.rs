#![no_std]
#![no_main]

use core::ffi::c_void;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[repr(transparent)]
pub struct EfiHandle(*mut c_void);

#[repr(C)]
pub struct EFI_TABLE_HEADER {
    pub signature: u64,
    pub revision: u32,
    pub header_size: u32,
    pub crc32: u32,
    _reserved: u32,
}

#[repr(C)]
pub struct EFI_SYSTEM_TABLE {
    pub header: EFI_TABLE_HEADER,
    pub firmware_vendor: *const u16,
    pub firmware_revision: u32,
    pub console_in_handle: EfiHandle,
    _con_in: usize,
    pub console_out_handle: EfiHandle,
    pub ConOut: EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,
    pub standard_error_handle: EfiHandle,
    _std_err: usize,
}

#[repr(C)]
pub struct EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL {
    pub _buf: u64,
    pub OutputString: unsafe extern "win64" fn(
        this: &EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,
        string: *const u16,
    ) -> EFI_STATUS,
    pub _buf2: [u64; 2],
    pub ClearScreen: unsafe extern "win64" fn(this: &EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL),
}

#[repr(usize)]
pub enum EFI_STATUS {
    SUCCESS = 0,
}

#[no_mangle]
pub extern "C" fn efi_main(_image: EfiHandle, SystemTable: EFI_SYSTEM_TABLE) -> ! {
    let string = "Hello UEFI\n".as_bytes();
    let mut buf = [0u16; 32];

    for i in 0..string.len() {
        buf[i] = string[i] as u16;
    }

    unsafe {
        (SystemTable.ConOut.ClearScreen)(&SystemTable.ConOut);
        (SystemTable.ConOut.OutputString)(&SystemTable.ConOut, buf.as_ptr());
    }
    loop {}
}

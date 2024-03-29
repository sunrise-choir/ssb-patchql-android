use common::*;
use wallet_wasm::*;

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_uchar};
use std::panic;
use std::slice;
use std::str;
use std::usize;

pub use constants::*;

fn handle_error<F: FnOnce() -> R + panic::UnwindSafe, R>(
    error: *mut *mut c_char,
    err_val: R,
    func: F,
) -> R {
    match handle_exception(func) {
        Ok(val) => val,
        Err(err) => {
            unsafe {
                *error = CString::new(err).unwrap().into_raw();
            }
            err_val
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn copy_string(cstr: *const c_char) -> *mut c_char {
    let vec = CStr::from_ptr(cstr).to_bytes_with_nul().to_vec();
    CString::from_vec_unchecked(vec).into_raw()
}

#[no_mangle]
pub extern "C" fn dealloc_string(ptr: *mut c_char) {
    dealloc_str(ptr);
}

#[no_mangle]
pub extern "C" fn init_cardano() {
    hide_exceptions();
}

#[no_mangle]
pub extern "C" fn wallet_from_enhanced_entropy_safe(
    entropy_ptr: *const c_uchar,
    entropy_size: usize,
    password_ptr: *const c_uchar,
    password_size: usize,
    out: *mut c_uchar,
    error: *mut *mut c_char,
) -> usize {
    handle_error(error, usize::MAX, || {
        wallet_from_enhanced_entropy(entropy_ptr, entropy_size, password_ptr, password_size, out)
    })
}

#[no_mangle]
pub extern "C" fn wallet_from_seed_safe(
    seed_ptr: *const c_uchar,
    out: *mut c_uchar,
    error: *mut *mut c_char,
) {
    handle_error(error, (), || wallet_from_seed(seed_ptr, out))
}

#[no_mangle]
pub extern "C" fn wallet_to_public_safe(
    xprv_ptr: *const c_uchar,
    out: *mut c_uchar,
    error: *mut *mut c_char,
) {
    handle_error(error, (), || wallet_to_public(xprv_ptr, out))
}

#[no_mangle]
pub extern "C" fn wallet_derive_private_safe(
    xprv_ptr: *const c_uchar,
    index: u32,
    out: *mut c_uchar,
    error: *mut *mut c_char,
) {
    handle_error(error, (), || wallet_derive_private(xprv_ptr, index, out))
}

#[no_mangle]
pub extern "C" fn wallet_derive_public_safe(
    xpub_ptr: *const c_uchar,
    index: u32,
    out: *mut c_uchar,
    error: *mut *mut c_char,
) -> bool {
    handle_error(error, false, || wallet_derive_public(xpub_ptr, index, out))
}

#[no_mangle]
pub extern "C" fn wallet_sign_safe(
    xprv_ptr: *const c_uchar,
    msg_ptr: *const c_uchar,
    msg_sz: usize,
    out: *mut c_uchar,
    error: *mut *mut c_char,
) {
    handle_error(error, (), || wallet_sign(xprv_ptr, msg_ptr, msg_sz, out))
}

#[no_mangle]
pub extern "C" fn xwallet_from_master_key_safe(
    input_ptr: *const c_uchar,
    output_ptr: *mut c_uchar,
    error: *mut *mut c_char,
) -> i32 {
    handle_error(error, -1, || xwallet_from_master_key(input_ptr, output_ptr))
}

#[no_mangle]
pub extern "C" fn xwallet_create_daedalus_mnemonic_safe(
    input_ptr: *const c_uchar,
    input_sz: usize,
    output_ptr: *mut c_uchar,
    error: *mut *mut c_char,
) -> i32 {
    handle_error(error, -1, || {
        xwallet_create_daedalus_mnemonic(input_ptr, input_sz, output_ptr)
    })
}

#[no_mangle]
pub extern "C" fn xwallet_account_safe(
    input_ptr: *const c_uchar,
    input_sz: usize,
    output_ptr: *mut c_uchar,
    error: *mut *mut c_char,
) -> i32 {
    handle_error(error, -1, || {
        xwallet_account(input_ptr, input_sz, output_ptr)
    })
}

#[no_mangle]
pub extern "C" fn xwallet_addresses_safe(
    input_ptr: *const c_uchar,
    input_sz: usize,
    output_ptr: *mut c_uchar,
    error: *mut *mut c_char,
) -> i32 {
    handle_error(error, -1, || {
        xwallet_addresses(input_ptr, input_sz, output_ptr)
    })
}

#[no_mangle]
pub extern "C" fn xwallet_checkaddress_safe(
    input_ptr: *const c_uchar,
    input_sz: usize,
    output_ptr: *mut c_uchar,
    error: *mut *mut c_char,
) -> i32 {
    handle_error(error, -1, || {
        let addr_str =
            unsafe { str::from_utf8_unchecked(slice::from_raw_parts(input_ptr, input_sz)) };
        let fixed_addr = convert_address_base58(addr_str);
        let addr: &[u8] = fixed_addr.as_bytes();

        xwallet_checkaddress(addr.as_ptr(), addr.len(), output_ptr)
    })
}

#[no_mangle]
pub extern "C" fn xwallet_spend_safe(
    input_ptr: *const c_uchar,
    input_sz: usize,
    output_ptr: *mut c_uchar,
    error: *mut *mut c_char,
) -> i32 {
    handle_error(error, -1, || xwallet_spend(input_ptr, input_sz, output_ptr))
}

#[no_mangle]
pub extern "C" fn xwallet_move_safe(
    input_ptr: *const c_uchar,
    input_sz: usize,
    output_ptr: *mut c_uchar,
    error: *mut *mut c_char,
) -> i32 {
    handle_error(error, -1, || xwallet_move(input_ptr, input_sz, output_ptr))
}

#[no_mangle]
pub extern "C" fn random_address_checker_new_safe(
    input_ptr: *const c_uchar,
    input_sz: usize,
    output_ptr: *mut c_uchar,
    error: *mut *mut c_char,
) -> i32 {
    handle_error(error, -1, || {
        random_address_checker_new(input_ptr, input_sz, output_ptr)
    })
}

#[no_mangle]
pub extern "C" fn random_address_checker_from_mnemonics_safe(
    input_ptr: *const c_uchar,
    input_sz: usize,
    output_ptr: *mut c_uchar,
    error: *mut *mut c_char,
) -> i32 {
    handle_error(error, -1, || {
        random_address_checker_from_mnemonics(input_ptr, input_sz, output_ptr)
    })
}

#[no_mangle]
pub extern "C" fn random_address_check_safe(
    input_ptr: *const c_uchar,
    input_sz: usize,
    output_ptr: *mut c_uchar,
    error: *mut *mut c_char,
) -> i32 {
    handle_error(error, -1, || {
        random_address_check(input_ptr, input_sz, output_ptr)
    })
}

#[no_mangle]
pub extern "C" fn encrypt_with_password_safe(
    password_ptr: *const c_uchar,
    password_sz: usize,
    salt_ptr: *const c_uchar,  // expect 32 bytes
    nonce_ptr: *const c_uchar, // expect 12 bytes
    data_ptr: *const c_uchar,
    data_sz: usize,
    output_ptr: *mut c_uchar,
    error: *mut *mut c_char,
) -> i32 {
    handle_error(error, -1, || {
        encrypt_with_password(
            password_ptr,
            password_sz,
            salt_ptr,
            nonce_ptr,
            data_ptr,
            data_sz,
            output_ptr,
        )
    })
}

#[no_mangle]
pub extern "C" fn decrypt_with_password_safe(
    password_ptr: *const c_uchar,
    password_sz: usize,
    data_ptr: *const c_uchar,
    data_sz: usize,
    output_ptr: *mut c_uchar,
    error: *mut *mut c_char,
) -> i32 {
    handle_error(error, -1, || {
        decrypt_with_password(password_ptr, password_sz, data_ptr, data_sz, output_ptr)
    })
}

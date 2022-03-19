use crate::Inventory;
use redis_module::raw;
use std::ffi::c_void;
use std::mem::size_of;
use std::os::raw::c_int;

pub extern "C" fn mem_usage(_value: *const c_void) -> usize {
    size_of::<Inventory>()
}

pub extern "C" fn free(value: *mut c_void) {
    unsafe { Box::from_raw(value.cast::<Inventory>()) };
}

pub extern "C" fn rdb_load(rdb: *mut raw::RedisModuleIO, _encver: c_int) -> *mut c_void {
    match raw::load_unsigned(rdb) {
        Ok(value) => Box::into_raw(Box::new(value)).cast::<c_void>(),
        Err(_) => std::ptr::null_mut(),
    }
}

pub extern "C" fn rdb_save(rdb: *mut raw::RedisModuleIO, value: *mut c_void) {
    unsafe { raw::save_unsigned(rdb, *(value as *const u64)) }
}

#[cfg(test)]
mod test {
    use crate::methods::mem_usage;
    use std::mem::size_of;

    #[test]
    fn mem_usage_() {
        assert_eq!(mem_usage(std::ptr::null()), size_of::<u64>());
    }
}

unsafe extern "C" {
    pub fn valkey_malloc(size: usize) -> *mut ();
    pub fn valkey_realloc(ptr: * mut (), size: usize) -> *mut ();
    pub fn valkey_free(ptr: *mut ());
}


#[macro_export]
macro_rules! zmalloc {
    ($a: expr) => {
        {
            valkey_malloc($a)
        }
    };
}


#[macro_export]
macro_rules! zrelloc {
    ($ptr: expr, $size: expr) => {
        {
            valkey_realloc($ptr, $size)
        }
    };
}


#[macro_export]
macro_rules! zfree {
    ($ptr: expr) => {
        {
            valkey_free($ptr);
        }
    };
}


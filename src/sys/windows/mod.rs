mod read_guard;
mod rw_lock;
mod utils;
mod write_guard;

pub use read_guard::RwLockReadGuard;
pub use rw_lock::RwLock;
pub use write_guard::RwLockWriteGuard;

#[allow(non_camel_case_types, non_snake_case)]
mod bindings {
    pub type BOOL = i32;
    pub type HANDLE = isize;

    pub const ERROR_LOCK_VIOLATION: u32 = 33;

    pub type LOCK_FILE_FLAGS = u32;
    pub const LOCKFILE_EXCLUSIVE_LOCK: LOCK_FILE_FLAGS = 2;
    pub const LOCKFILE_FAIL_IMMEDIATELY: LOCK_FILE_FLAGS = 1;

    #[repr(C)]
    pub struct OVERLAPPED_0_0 {
        pub Offset: u32,
        pub OffsetHigh: u32,
    }

    #[repr(C)]
    pub union OVERLAPPED_0 {
        pub Anonymous: std::mem::ManuallyDrop<OVERLAPPED_0_0>,
        pub Pointer: *mut std::ffi::c_void,
    }

    #[repr(C)]
    pub struct OVERLAPPED {
        pub Internal: usize,
        pub InternalHigh: usize,
        pub Anonymous: OVERLAPPED_0,
        pub hEvent: HANDLE,
    }

    #[link(name = "kernel32", kind = "raw-dylib")]
    extern "system" {
        pub fn UnlockFile(
            hFile: HANDLE,
            dwFileOffsetLow: u32,
            dwFileOffsetHigh: u32,
            nNumberOfBytesToUnlockLow: u32,
            nNumberOfBytesToUnlockHigh: u32,
        ) -> BOOL;
        pub fn LockFileEx(
            hFile: HANDLE,
            dwFlags: LOCK_FILE_FLAGS,
            dwReserved: u32,
            nNumberOfBytesToLockLow: u32,
            nNumberOfBytesToLockHigh: u32,
            lpOverlapped: *mut OVERLAPPED,
        ) -> BOOL;
    }
}

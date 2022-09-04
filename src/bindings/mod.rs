use windows::Win32::{System::Threading::{RTL_USER_PROCESS_PARAMETERS}, Foundation::UNICODE_STRING};
extern "system" {
    fn GetLastError() -> u32;
}

macro_rules! UNION {
    ($(#[$attrs:meta])* union $name:ident {
        [$stype:ty; $ssize:expr],
        $($variant:ident $variant_mut:ident: $ftype:ty,)+
    }) => (
        #[repr(C)] $(#[$attrs])*
        pub struct $name([$stype; $ssize]);
        impl Copy for $name {}
        impl Clone for $name {
            #[inline]
            fn clone(&self) -> $name { *self }
        }
        #[cfg(feature = "impl-default")]
        impl Default for $name {
            #[inline]
            fn default() -> $name { unsafe { $crate::_core::mem::zeroed() } }
        }
        impl $name {$(
            #[inline]
            pub unsafe fn $variant(&self) -> &$ftype {
                &*(self as *const _ as *const $ftype)
            }
            #[inline]
            pub unsafe fn $variant_mut(&mut self) -> &mut $ftype {
                &mut *(self as *mut _ as *mut $ftype)
            }
        )+}
    );
    ($(#[$attrs:meta])* union $name:ident {
        [$stype32:ty; $ssize32:expr] [$stype64:ty; $ssize64:expr],
        $($variant:ident $variant_mut:ident: $ftype:ty,)+
    }) => (
        #[repr(C)] $(#[$attrs])* #[cfg(target_pointer_width = "32")]
        pub struct $name([$stype32; $ssize32]);
        #[repr(C)] $(#[$attrs])* #[cfg(target_pointer_width = "64")]
        pub struct $name([$stype64; $ssize64]);
        impl Copy for $name {}
        impl Clone for $name {
            #[inline]
            fn clone(&self) -> $name { *self }
        }
        #[cfg(feature = "impl-default")]
        impl Default for $name {
            #[inline]
            fn default() -> $name { unsafe { $crate::_core::mem::zeroed() } }
        }
        impl $name {$(
            #[inline]
            pub unsafe fn $variant(&self) -> &$ftype {
                &*(self as *const _ as *const $ftype)
            }
            #[inline]
            pub unsafe fn $variant_mut(&mut self) -> &mut $ftype {
                &mut *(self as *mut _ as *mut $ftype)
            }
        )+}
    );
}

pub mod ctypes {
    #[cfg(feature = "std")]
    pub use std::os::raw::c_void;
    #[cfg(not(feature = "std"))]
    pub enum c_void {}
    pub type c_char = i8;
    pub type c_schar = i8;
    pub type c_uchar = u8;
    pub type c_short = i16;
    pub type c_ushort = u16;
    pub type c_int = i32;
    pub type c_uint = u32;
    pub type c_long = i32;
    pub type c_ulong = u32;
    pub type c_longlong = i64;
    pub type c_ulonglong = u64;
    pub type c_float = f32;
    pub type c_double = f64;
    pub type __int8 = i8;
    pub type __uint8 = u8;
    pub type __int16 = i16;
    pub type __uint16 = u16;
    pub type __int32 = i32;
    pub type __uint32 = u32;
    pub type __int64 = i64;
    pub type __uint64 = u64;
    pub type wchar_t = u16;
}

pub type BYTE = ctypes::c_uchar;
pub type PBYTE = *mut BYTE;
pub type LONG = ctypes::c_long;
pub type NTSTATUS = LONG;
pub type WCHAR = ctypes::wchar_t;
pub type PWCHAR = *mut WCHAR; // *mut u16
pub type WORD = ctypes::c_char;
pub type DWORD = u32;
pub type DWORD64 = ctypes::__uint64;
pub type HANDLE = *mut ctypes::c_void;
pub type ULONG_PTR = usize;
pub type PVOID = *mut ctypes::c_void;
pub type ULONG = ctypes::c_ulong;
pub type SHORT = ctypes::c_short;
pub type BOOLEAN = ctypes::c_uchar;
pub type LONGLONG = ctypes::__int64;

#[repr(C)]
struct LARGE_INTEGER_u {
    LowPart: ULONG,
    HighPart: LONG,
}

#[repr(C)]
struct LARGE_INTEGER_s {
    LowPart: ULONG,
    HighPart: LONG,
}

UNION!{union LARGE_INTEGER {
    [i64; 1],
    s s_mut: LARGE_INTEGER_s,
    u u_mut: LARGE_INTEGER_u,
    QuadPart QuadPart_mut: LONGLONG,
}}

#[repr(transparent)]
pub struct PWSTR(pub *mut u16);

pub type PPS_POST_PROCESS_INIT_ROUTINE = Option<unsafe extern "system" fn()>;

#[repr(C)]
pub struct LIST_ENTRY {
    pub Flink: *mut LIST_ENTRY,
    pub Blink: *mut LIST_ENTRY,
}

#[repr(C)]
pub struct _PEB_LDR_DATA {
    pub Length: ULONG,
    pub Initialized: ULONG,
    pub SsHandle: PVOID,
    pub InLoadOrderModuleList: LIST_ENTRY,
    pub InMemoryOrderModuleList: LIST_ENTRY,
    pub InInitializationOrderModuleList: LIST_ENTRY,
}
pub type PPEB_LDR_DATA = *mut _PEB_LDR_DATA;

#[repr(C)]
pub struct _PEB {
	pub InheritedAddressSpace: BOOLEAN                 ,
	pub BeingDebugged: BOOLEAN                 ,
	pub Spare: BOOLEAN                 ,
	pub Mutant:HANDLE                 ,
	pub ImageBase:PVOID                  ,
	pub LoaderData: PPEB_LDR_DATA                              ,
	pub ProcessParameters:	*mut RTL_USER_PROCESS_PARAMETERS                              ,
	pub SubSystemData:      PVOID                                      ,
	pub ProcessHeap:PVOID                   ,
	pub FastPebLock:PVOID                   ,
	pub FastPebLockRoutine:PVOID                   ,
	pub FastPebUnlockRoutine:PVOID                   ,
	pub EnvironmentUpdateCount:ULONG,
	pub KernelCallbackTable: *mut PVOID,
	pub EventLogSection:PVOID                   ,
	pub EventLog:PVOID                   ,
	pub FreeList:PVOID                   ,
	pub TlsExpansionCounter:ULONG,
	pub TlsBitmap:PVOID                   ,
	pub TlsBitmapBits: [ULONG; 0x2],
	pub ReadOnlySharedMemoryBase:PVOID                   ,
	pub ReadOnlySharedMemoryHeap:PVOID                   ,
	pub ReadOnlyStaticServerData:PVOID                   ,
	pub AnsiCodePageData:PVOID                   ,
	pub OemCodePageData:PVOID                   ,
	pub UnicodeCaseTableData:PVOID                   ,
	pub NumberOfProcessors:ULONG,
	pub NtGlobalFlag:ULONG,
	pub Spare2: [BYTE; 0x4],
	pub CriticalSectionTimeout:LARGE_INTEGER,
	pub HeapSegmentReserve:ULONG                   ,
	pub HeapSegmentCommit:ULONG                   ,
	pub HeapDeCommitTotalFreeThreshold:ULONG                   ,
	pub HeapDeCommitFreeBlockThreshold:ULONG                   ,
	pub NumberOfHeaps:ULONG                   ,
	pub MaximumNumberOfHeaps:ULONG                   ,
	pub ProcessHeaps: *mut *mut PVOID,
	pub GdiSharedHandleTable:PVOID                   ,
	pub ProcessStarterHelper:PVOID                   ,
	pub GdiDCAttributeList:PVOID                   ,
	pub LoaderLock:PVOID                   ,
	pub OSMajorVersion:ULONG                   ,
	pub OSMinorVersion:ULONG                   ,
	pub OSBuildNumber:ULONG                   ,
	pub OSPlatformId:ULONG                   ,
	pub ImageSubSystem:ULONG                   ,
	pub ImageSubSystemMajorVersion:ULONG                   ,
	pub ImageSubSystemMinorVersion:ULONG                   ,
	pub GdiHandleBuffer: [ULONG; 0x22]                   ,
	pub PostProcessInitRoutine:ULONG                   ,
	pub TlsExpansionBitmap:ULONG                   ,
	pub TlsExpansionBitmapBits: [BYTE; 0x80]                  ,
	pub SessionId:ULONG,
}     
pub type PPEB = *mut _PEB;

#[repr(C)]
pub struct _LDR_MODULE {
    pub InLoadOrderModuleList: LIST_ENTRY,
    pub InMemoryOrderModuleList: LIST_ENTRY,
    pub InInitializationOrderModuleList: LIST_ENTRY,
    pub BaseAddress: PVOID,
    pub EntryPoint: PVOID,
    pub SizeOfImage: ULONG,
    pub FullDllName: UNICODE_STRING,
    pub BaseDllName: UNICODE_STRING,
    pub Flags: ULONG,
    pub LoadCount: SHORT,
    pub TlsIndex: SHORT,
    pub HashTableEntry: LIST_ENTRY,
    pub TimeDateStamp: ULONG
}

pub type PLDR_MODULE = *mut _LDR_MODULE;
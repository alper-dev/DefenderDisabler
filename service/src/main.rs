use chrono::Local;
use std::fs::OpenOptions;
use std::io::Write;
use std::ptr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

type HANDLE = *mut std::ffi::c_void;
type HWND = *mut std::ffi::c_void;
type HKEY = *mut std::ffi::c_void;
type HINSTANCE = *mut std::ffi::c_void;
type HICON = *mut std::ffi::c_void;
type HCURSOR = *mut std::ffi::c_void;
type HBRUSH = *mut std::ffi::c_void;
type HMENU = *mut std::ffi::c_void;
type HMODULE = *mut std::ffi::c_void;
type BOOL = i32;
type DWORD = u32;
type UINT = u32;
type WPARAM = usize;
type LPARAM = isize;
type LRESULT = isize;
type LONG = i32;
type LPCWSTR = *const u16;
type LPWSTR = *mut u16;
type LPCVOID = *const std::ffi::c_void;
type LPDWORD = *mut DWORD;
type LPBYTE = *mut u8;
type PHKEY = *mut HKEY;

const FALSE: BOOL = 0;
const TRUE: BOOL = 1;
const ERROR_SUCCESS: LONG = 0;
const ERROR_ALREADY_EXISTS: DWORD = 183;
const WAIT_OBJECT_0: DWORD = 0;
const INVALID_HANDLE_VALUE: HANDLE = -1isize as HANDLE;
const HKEY_LOCAL_MACHINE: HKEY = 0x80000002 as HKEY;
const KEY_READ: DWORD = 0x20019;
const KEY_SET_VALUE: DWORD = 0x0002;
const KEY_NOTIFY: DWORD = 0x0010;
const REG_DWORD: DWORD = 4;
const REG_NOTIFY_CHANGE_NAME: DWORD = 1;
const REG_NOTIFY_CHANGE_LAST_SET: DWORD = 4;
const PIPE_ACCESS_DUPLEX: DWORD = 0x00000003;
const PIPE_TYPE_MESSAGE: DWORD = 0x00000004;
const PIPE_READMODE_MESSAGE: DWORD = 0x00000002;
const PIPE_WAIT: DWORD = 0x00000000;
const GENERIC_READ: DWORD = 0x80000000;
const GENERIC_WRITE: DWORD = 0x40000000;
const OPEN_EXISTING: DWORD = 3;
const INFINITE: DWORD = 0xFFFFFFFF;
const WS_OVERLAPPEDWINDOW: DWORD = 0x00CF0000;
const CW_USEDEFAULT: i32 = 0x80000000u32 as i32;
const WM_USER: u32 = 0x0400;
const WM_DESTROY: u32 = 0x0002;
const WM_COMMAND: u32 = 0x0111;
const WM_RBUTTONUP: u32 = 0x0205;
const WM_CONTEXTMENU: u32 = 0x007B;
const WM_LBUTTONUP: u32 = 0x0202;
const MF_STRING: u32 = 0x00000000;
const MF_SEPARATOR: u32 = 0x00000800;
const MF_DISABLED: u32 = 0x00000002;
const MF_GRAYED: u32 = 0x00000001;
const TPM_BOTTOMALIGN: u32 = 0x0020;
const TPM_LEFTALIGN: u32 = 0x0000;
const SW_SHOW: i32 = 5;
const NIM_ADD: u32 = 0x00000000;
const NIM_MODIFY: u32 = 0x00000001;
const NIM_DELETE: u32 = 0x00000002;
const NIF_ICON: u32 = 0x00000002;
const NIF_MESSAGE: u32 = 0x00000001;
const NIF_TIP: u32 = 0x00000004;
const NIF_INFO: u32 = 0x00000010;
const NIIF_INFO: u32 = 0x00000001;
const IDI_APPLICATION: LPCWSTR = 32512 as LPCWSTR;

// Service Control Manager constants
const SERVICE_WIN32_OWN_PROCESS: DWORD = 0x00000010;
const SERVICE_ACCEPT_STOP: DWORD = 0x00000001;
const SERVICE_ACCEPT_SHUTDOWN: DWORD = 0x00000004;
const SERVICE_RUNNING: DWORD = 0x00000004;
const SERVICE_STOPPED: DWORD = 0x00000001;
const SERVICE_CONTROL_STOP: DWORD = 0x00000001;
const SERVICE_CONTROL_SHUTDOWN: DWORD = 0x00000005;
const NO_ERROR: DWORD = 0;

type LPHANDLER_FUNCTION_EX = Option<unsafe extern "system" fn(DWORD, DWORD, *mut std::ffi::c_void, *mut std::ffi::c_void) -> DWORD>;

#[repr(C)]
struct SERVICE_TABLE_ENTRYW {
    lpServiceName: LPCWSTR,
    lpServiceProc: Option<unsafe extern "system" fn(DWORD, *mut LPCWSTR)>,
}

#[repr(C)]
struct SERVICE_STATUS {
    dwServiceType: DWORD,
    dwCurrentState: DWORD,
    dwControlsAccepted: DWORD,
    dwWin32ExitCode: DWORD,
    dwServiceSpecificExitCode: DWORD,
    dwCheckPoint: DWORD,
    dwWaitHint: DWORD,
}

#[repr(C)]
struct WNDCLASSEXW {
    cbSize: UINT,
    style: UINT,
    lpfnWndProc: Option<unsafe extern "system" fn(HWND, UINT, WPARAM, LPARAM) -> LRESULT>,
    cbClsExtra: i32,
    cbWndExtra: i32,
    hInstance: HINSTANCE,
    hIcon: HICON,
    hCursor: HCURSOR,
    hbrBackground: HBRUSH,
    lpszMenuName: LPCWSTR,
    lpszClassName: LPCWSTR,
    hIconSm: HICON,
}

#[repr(C)]
struct POINT {
    x: i32,
    y: i32,
}

#[repr(C)]
struct MSG {
    hwnd: HWND,
    message: UINT,
    wParam: WPARAM,
    lParam: LPARAM,
    time: DWORD,
    pt: POINT,
    lPrivate: DWORD,
}

#[repr(C)]
struct NOTIFYICONDATAW {
    cbSize: DWORD,
    hWnd: HWND,
    uID: UINT,
    uFlags: UINT,
    uCallbackMessage: UINT,
    hIcon: HICON,
    szTip: [u16; 128],
    dwState: DWORD,
    dwStateMask: DWORD,
    szInfo: [u16; 256],
    uVersion: UINT,
    szInfoTitle: [u16; 64],
    dwInfoFlags: DWORD,
    guidItem: [u8; 16],
    hBalloonIcon: HICON,
}

#[link(name = "kernel32")]
extern "system" {
    fn GetModuleHandleW(lpModuleName: LPCWSTR) -> HMODULE;
    fn GetLastError() -> DWORD;
    fn CreateEventW(lpEventAttributes: *mut std::ffi::c_void, bManualReset: BOOL, bInitialState: BOOL, lpName: LPCWSTR) -> HANDLE;
    fn WaitForSingleObject(hHandle: HANDLE, dwMilliseconds: DWORD) -> DWORD;
    fn CloseHandle(hObject: HANDLE) -> BOOL;
    fn CreateNamedPipeW(lpName: LPCWSTR, dwOpenMode: DWORD, dwPipeMode: DWORD, nMaxInstances: DWORD, nOutBufferSize: DWORD, nInBufferSize: DWORD, nDefaultTimeOut: DWORD, lpSecurityAttributes: *mut std::ffi::c_void) -> HANDLE;
    fn ConnectNamedPipe(hNamedPipe: HANDLE, lpOverlapped: *mut std::ffi::c_void) -> BOOL;
    fn ReadFile(hFile: HANDLE, lpBuffer: *mut std::ffi::c_void, nNumberOfBytesToRead: DWORD, lpNumberOfBytesRead: LPDWORD, lpOverlapped: *mut std::ffi::c_void) -> BOOL;
    fn WriteFile(hFile: HANDLE, lpBuffer: LPCVOID, nNumberOfBytesToWrite: DWORD, lpNumberOfBytesWritten: LPDWORD, lpOverlapped: *mut std::ffi::c_void) -> BOOL;
    fn CreateFileW(lpFileName: LPCWSTR, dwDesiredAccess: DWORD, dwShareMode: DWORD, lpSecurityAttributes: *mut std::ffi::c_void, dwCreationDisposition: DWORD, dwFlagsAndAttributes: DWORD, hTemplateFile: HANDLE) -> HANDLE;
}

#[link(name = "advapi32")]
extern "system" {
    fn RegOpenKeyExW(hKey: HKEY, lpSubKey: LPCWSTR, ulOptions: DWORD, samDesired: DWORD, phkResult: PHKEY) -> LONG;
    fn RegCloseKey(hKey: HKEY) -> LONG;
    fn RegSetValueExW(hKey: HKEY, lpValueName: LPCWSTR, Reserved: DWORD, dwType: DWORD, lpData: *const u8, cbData: DWORD) -> LONG;
    fn RegQueryValueExW(hKey: HKEY, lpValueName: LPCWSTR, lpReserved: *mut DWORD, lpType: *mut DWORD, lpData: LPBYTE, lpcbData: LPDWORD) -> LONG;
    fn RegNotifyChangeKeyValue(hKey: HKEY, bWatchSubtree: BOOL, dwNotifyFilter: DWORD, hEvent: HANDLE, fAsynchronous: BOOL) -> LONG;
    fn StartServiceCtrlDispatcherW(lpServiceTable: *const SERVICE_TABLE_ENTRYW) -> BOOL;
    fn RegisterServiceCtrlHandlerExW(lpServiceName: LPCWSTR, lpHandlerProc: LPHANDLER_FUNCTION_EX, lpContext: *mut std::ffi::c_void) -> HANDLE;
    fn SetServiceStatus(hServiceStatus: HANDLE, lpServiceStatus: *const SERVICE_STATUS) -> BOOL;
}

#[link(name = "user32")]
extern "system" {
    fn RegisterClassExW(lpWndClass: *const WNDCLASSEXW) -> u16;
    fn UnregisterClassW(lpClassName: LPCWSTR, hInstance: HINSTANCE) -> BOOL;
    fn CreateWindowExW(dwExStyle: DWORD, lpClassName: LPCWSTR, lpWindowName: LPCWSTR, dwStyle: DWORD, x: i32, y: i32, nWidth: i32, nHeight: i32, hWndParent: HWND, hMenu: HMENU, hInstance: HINSTANCE, lpParam: *mut std::ffi::c_void) -> HWND;
    fn DestroyWindow(hWnd: HWND) -> BOOL;
    fn DefWindowProcW(hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT;
    fn PostQuitMessage(nExitCode: i32);
    fn GetMessageW(lpMsg: *mut MSG, hWnd: HWND, wMsgFilterMin: UINT, wMsgFilterMax: UINT) -> BOOL;
    fn TranslateMessage(lpMsg: *const MSG) -> BOOL;
    fn DispatchMessageW(lpMsg: *const MSG) -> LRESULT;
    fn SetForegroundWindow(hWnd: HWND) -> BOOL;
    fn TrackPopupMenu(hMenu: HMENU, uFlags: UINT, x: i32, y: i32, nReserved: i32, hWnd: HWND, prcRect: *const std::ffi::c_void) -> BOOL;
    fn GetCursorPos(lpPoint: *mut POINT) -> BOOL;
    fn CreatePopupMenu() -> HMENU;
    fn DestroyMenu(hMenu: HMENU) -> BOOL;
    fn AppendMenuW(hMenu: HMENU, uFlags: UINT, uIDNewItem: usize, lpNewItem: LPCWSTR) -> BOOL;
    fn LoadIconW(hInstance: HINSTANCE, lpIconName: LPCWSTR) -> HICON;
}

#[link(name = "shell32")]
extern "system" {
    fn Shell_NotifyIconW(dwMessage: UINT, lpData: *const NOTIFYICONDATAW) -> BOOL;
    fn ShellExecuteW(hwnd: HWND, lpOperation: LPCWSTR, lpFile: LPCWSTR, lpParameters: LPCWSTR, lpDirectory: LPCWSTR, nShowCmd: i32) -> HINSTANCE;
}

const LOG_PATH: &str = r"C:\Program Files\Defender Disabler\defender-disabler.log";
const WM_TRAYICON: u32 = WM_USER + 1;
const ID_STATUS: usize = 1;
const ID_VIEW_LOGS: usize = 2;
const ID_EXIT: usize = 3;
const RETRY_INTERVAL_SECS: u64 = 30;
const TOAST_COOLDOWN_SECS: i64 = 300;

static DEFENDER_DISABLED: AtomicBool = AtomicBool::new(false);

fn log(msg: &str) {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
    let line = format!("[{}] {}\n", timestamp, msg);
    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(LOG_PATH) {
        let _ = file.write_all(line.as_bytes());
    }
}

fn to_wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

fn show_toast(title: &str, message: &str) {
    unsafe {
        let class_name = to_wide("DefenderDisablerToast");
        let wc = WNDCLASSEXW {
            cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
            style: 0,
            lpfnWndProc: Some(def_window_proc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: GetModuleHandleW(ptr::null()),
            hIcon: ptr::null_mut(),
            hCursor: ptr::null_mut(),
            hbrBackground: ptr::null_mut(),
            lpszMenuName: ptr::null(),
            lpszClassName: class_name.as_ptr(),
            hIconSm: ptr::null_mut(),
        };
        RegisterClassExW(&wc);

        let hwnd = CreateWindowExW(
            0,
            class_name.as_ptr(),
            to_wide("Defender Disabler").as_ptr(),
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            ptr::null_mut(),
            ptr::null_mut(),
            GetModuleHandleW(ptr::null()),
            ptr::null_mut(),
        );

        if !hwnd.is_null() {
            let title_w = to_wide(title);
            let message_w = to_wide(message);

            let mut nid: NOTIFYICONDATAW = std::mem::zeroed();
            nid.cbSize = std::mem::size_of::<NOTIFYICONDATAW>() as u32;
            nid.hWnd = hwnd;
            nid.uID = 1;
            nid.uFlags = NIF_INFO;
            nid.szInfoTitle[..title_w.len().min(63)].copy_from_slice(&title_w[..title_w.len().min(63)]);
            nid.szInfo[..message_w.len().min(255)].copy_from_slice(&message_w[..message_w.len().min(255)]);
            nid.dwInfoFlags = NIIF_INFO;

            Shell_NotifyIconW(NIM_ADD, &nid);
            Shell_NotifyIconW(NIM_DELETE, &nid);
            DestroyWindow(hwnd);
        }
        UnregisterClassW(class_name.as_ptr(), GetModuleHandleW(ptr::null()));
    }
}

fn set_dword_value(hkey_path: &str, value_name: &str, value: u32) -> bool {
    unsafe {
        let mut hkey: HKEY = ptr::null_mut();
        let key = to_wide(hkey_path);
        if RegOpenKeyExW(HKEY_LOCAL_MACHINE, key.as_ptr(), 0, KEY_SET_VALUE, &mut hkey) == ERROR_SUCCESS {
            let name = to_wide(value_name);
            let result = RegSetValueExW(hkey, name.as_ptr(), 0, REG_DWORD, &value as *const u32 as *const u8, 4);
            RegCloseKey(hkey);
            return result == ERROR_SUCCESS;
        }
        false
    }
}

fn check_tamper_protection() -> u32 {
    unsafe {
        let mut hkey: HKEY = ptr::null_mut();
        let mut value: u32 = 0;
        let mut size = std::mem::size_of::<u32>() as u32;

        let key = to_wide(r"SOFTWARE\Microsoft\Windows Defender\Features");
        if RegOpenKeyExW(HKEY_LOCAL_MACHINE, key.as_ptr(), 0, KEY_READ, &mut hkey) == ERROR_SUCCESS {
            let name = to_wide("TamperProtection");
            RegQueryValueExW(hkey, name.as_ptr(), ptr::null_mut(), ptr::null_mut(), &mut value as *mut u32 as *mut u8, &mut size);
            RegCloseKey(hkey);
        }
        value
    }
}

fn disable_defender() -> bool {
    // Check Tamper Protection first
    let tp = check_tamper_protection();
    if tp >= 4 {
        log("WARNING: Tamper Protection is ON - registry writes may be blocked");
    }

    // Critical: must set DisableRealtimeMonitoring
    let rtp_key = r"SOFTWARE\Microsoft\Windows Defender\Real-Time Protection";
    if !set_dword_value(rtp_key, "DisableRealtimeMonitoring", 1) {
        log("Failed to set DisableRealtimeMonitoring");
        return false;
    }

    // Best-effort: these may fail if Tamper Protection blocks them, that's OK
    set_dword_value(rtp_key, "DisableBehaviorMonitoring", 1);
    set_dword_value(rtp_key, "DisableIOAVProtection", 1);
    set_dword_value(rtp_key, "DisableScriptScanning", 1);

    let main_key = r"SOFTWARE\Microsoft\Windows Defender";
    set_dword_value(main_key, "DisableAntiSpyware", 1);
    set_dword_value(main_key, "DisableAntiVirus", 1);

    let policy_key = r"SOFTWARE\Policies\Microsoft\Windows Defender";
    set_dword_value(policy_key, "DisableAntiSpyware", 1);

    let policy_rtp_key = r"SOFTWARE\Policies\Microsoft\Windows Defender\Real-Time Protection";
    set_dword_value(policy_rtp_key, "DisableRealtimeMonitoring", 1);
    set_dword_value(policy_rtp_key, "DisableBehaviorMonitoring", 1);

    true
}

fn read_dword_value(hkey_path: &str, value_name: &str) -> Option<u32> {
    unsafe {
        let mut hkey: HKEY = ptr::null_mut();
        let mut value: u32 = 0;
        let mut size = std::mem::size_of::<u32>() as u32;

        let key = to_wide(hkey_path);
        if RegOpenKeyExW(HKEY_LOCAL_MACHINE, key.as_ptr(), 0, KEY_READ, &mut hkey) == ERROR_SUCCESS {
            let name = to_wide(value_name);
            let result = RegQueryValueExW(hkey, name.as_ptr(), ptr::null_mut(), ptr::null_mut(), &mut value as *mut u32 as *mut u8, &mut size);
            RegCloseKey(hkey);
            if result == ERROR_SUCCESS {
                return Some(value);
            }
        }
        None
    }
}

fn check_defender_status() -> bool {
    // Primary check: if DisableRealtimeMonitoring is 1, Defender is disabled
    let rtp_key = r"SOFTWARE\Microsoft\Windows Defender\Real-Time Protection";
    let realtime = read_dword_value(rtp_key, "DisableRealtimeMonitoring").unwrap_or(0);
    
    // Defender is enabled if real-time protection is NOT disabled (value is 0 or missing)
    realtime == 0
}

fn monitor_registry_keys(running: Arc<AtomicBool>) {
    unsafe {
        let keys_to_monitor = [
            r"SOFTWARE\Microsoft\Windows Defender\Real-Time Protection",
            r"SOFTWARE\Microsoft\Windows Defender\Features",
            r"SOFTWARE\Policies\Microsoft\Windows Defender",
            r"SOFTWARE\Policies\Microsoft\Windows Defender\Real-Time Protection",
            r"SOFTWARE\Microsoft\Windows Defender",
        ];

        while running.load(Ordering::Relaxed) {
            let mut handles = Vec::new();

            for key_path in &keys_to_monitor {
                let mut hkey: HKEY = ptr::null_mut();
                let key_w = to_wide(key_path);
                if RegOpenKeyExW(HKEY_LOCAL_MACHINE, key_w.as_ptr(), 0, KEY_NOTIFY, &mut hkey) == ERROR_SUCCESS {
                    handles.push(hkey);
                }
            }

            if handles.is_empty() {
                log("Failed to open any registry keys for monitoring");
                thread::sleep(std::time::Duration::from_secs(5));
                continue;
            }

            // Wait for any key to change
            let event = CreateEventW(ptr::null_mut(), TRUE, FALSE, ptr::null());
            let mut any_changed = false;

            for &hkey in &handles {
                if RegNotifyChangeKeyValue(
                    hkey,
                    TRUE,
                    REG_NOTIFY_CHANGE_NAME | REG_NOTIFY_CHANGE_LAST_SET,
                    event,
                    TRUE,
                ) == ERROR_SUCCESS
                {
                    let wait_result = WaitForSingleObject(event, 1000);
                    if wait_result == WAIT_OBJECT_0 {
                        any_changed = true;
                        break;
                    }
                }
            }

            CloseHandle(event);

            for hkey in handles {
                RegCloseKey(hkey);
            }

            if any_changed {
                log("Registry change detected, checking Defender status...");
                let is_enabled = check_defender_status();

                if is_enabled {
                    log("Defender is enabled, attempting to disable...");
                    if disable_defender() {
                        log("Defender disabled successfully");
                        show_toast("Defender Disabler", "Defender has been disabled");
                        DEFENDER_DISABLED.store(true, Ordering::Relaxed);
                    } else {
                        log("Failed to disable Defender");
                        show_toast(
                            "Defender Disabler",
                            "Failed to disable Defender - Tamper Protection may be enabled",
                        );
                    }
                } else {
                    log("Defender is already disabled");
                    DEFENDER_DISABLED.store(true, Ordering::Relaxed);
                }
            }
        }
    }
}

fn handle_pipe_commands(running: Arc<AtomicBool>) {
    unsafe {
        let pipe_name = to_wide(r"\\.\pipe\DefenderDisabler");

        while running.load(Ordering::Relaxed) {
            let pipe = CreateNamedPipeW(
                pipe_name.as_ptr(),
                PIPE_ACCESS_DUPLEX,
                PIPE_TYPE_MESSAGE | PIPE_READMODE_MESSAGE | PIPE_WAIT,
                1,
                1024,
                1024,
                0,
                ptr::null_mut(),
            );

            if pipe == INVALID_HANDLE_VALUE {
                thread::sleep(std::time::Duration::from_secs(1));
                continue;
            }

            if ConnectNamedPipe(pipe, ptr::null_mut()) != 0 {
                let mut buffer = [0u8; 256];
                let mut bytes_read = 0;

                if ReadFile(pipe, buffer.as_mut_ptr() as *mut _, buffer.len() as u32, &mut bytes_read, ptr::null_mut()) != 0 {
                    let command = String::from_utf8_lossy(&buffer[..bytes_read as usize]);

                    match command.trim() {
                        "STATUS" => {
                            let status = if DEFENDER_DISABLED.load(Ordering::Relaxed) {
                                b"DISABLED".as_ref()
                            } else {
                                b"ENABLED".as_ref()
                            };
                            let mut bytes_written = 0;
                            WriteFile(pipe, status.as_ptr() as *const _, status.len() as u32, &mut bytes_written, ptr::null_mut());
                        }
                        "EXIT" => {
                            running.store(false, Ordering::Relaxed);
                        }
                        _ => {}
                    }
                }
            }

            CloseHandle(pipe);
        }
    }
}

static mut SERVICE_STATUS_HANDLE: HANDLE = ptr::null_mut();
static mut SERVICE_STATUS: SERVICE_STATUS = SERVICE_STATUS {
    dwServiceType: SERVICE_WIN32_OWN_PROCESS,
    dwCurrentState: SERVICE_STOPPED,
    dwControlsAccepted: 0,
    dwWin32ExitCode: 0,
    dwServiceSpecificExitCode: 0,
    dwCheckPoint: 0,
    dwWaitHint: 0,
};

fn report_service_status(state: DWORD, exit_code: DWORD, wait_hint: DWORD) {
    unsafe {
        SERVICE_STATUS.dwCurrentState = state;
        SERVICE_STATUS.dwWin32ExitCode = exit_code;
        SERVICE_STATUS.dwWaitHint = wait_hint;
        if state == SERVICE_RUNNING {
            SERVICE_STATUS.dwControlsAccepted = SERVICE_ACCEPT_STOP | SERVICE_ACCEPT_SHUTDOWN;
        } else {
            SERVICE_STATUS.dwControlsAccepted = 0;
        }
        SetServiceStatus(SERVICE_STATUS_HANDLE, &SERVICE_STATUS);
    }
}

unsafe extern "system" fn service_ctrl_handler(
    dwcontrol: DWORD,
    _dwEventType: DWORD,
    _lpEventData: *mut std::ffi::c_void,
    _lpContext: *mut std::ffi::c_void,
) -> DWORD {
    match dwcontrol {
        SERVICE_CONTROL_STOP | SERVICE_CONTROL_SHUTDOWN => {
            log("Service stop requested");
            report_service_status(SERVICE_STOPPED, 0, 0);
        }
        _ => {}
    }
    NO_ERROR
}

unsafe extern "system" fn service_main(_dwNumServicesArgs: DWORD, _lpServiceArgVectors: *mut LPCWSTR) {
    SERVICE_STATUS_HANDLE = RegisterServiceCtrlHandlerExW(
        to_wide("DefenderDisabler\0").as_ptr(),
        Some(service_ctrl_handler),
        ptr::null_mut(),
    );

    if SERVICE_STATUS_HANDLE.is_null() {
        log("Failed to register service control handler");
        return;
    }

    report_service_status(0x00000002, 0, 3000); // SERVICE_START_PENDING

    // Run the actual service logic
    run_service();

    report_service_status(SERVICE_STOPPED, 0, 0);
}

fn run_service() {
    log("Defender Disabler service started");

    // Log Tamper Protection status for diagnostics
    let tp = check_tamper_protection();
    match tp {
        0 => log("Tamper Protection: OFF"),
        1..=3 => log("Tamper Protection: PARTIAL"),
        4 => log("Tamper Protection: ON (registry value 0x4)"),
        5 => log("Tamper Protection: ON (strict mode, registry value 0x5)"),
        _ => log(&format!("Tamper Protection: UNKNOWN ({})", tp)),
    }

    report_service_status(SERVICE_RUNNING, 0, 0);
    log("Service running");

    let running = Arc::new(AtomicBool::new(true));

    // Spawn Defender disable thread (handles retries in background)
    let running_clone = running.clone();
    let disable_handle = thread::spawn(move || {
        if check_defender_status() {
            log("Defender is enabled, attempting to disable...");
            if disable_defender() {
                log("Defender disabled successfully");
                show_toast("Defender Disabler", "Defender has been disabled");
                DEFENDER_DISABLED.store(true, Ordering::Relaxed);
            } else {
                log("Failed to disable Defender - Tamper Protection may be blocking");
                show_toast(
                    "Defender Disabler",
                    "Failed to disable Defender - Please disable Tamper Protection",
                );

                // Retry in background
                let mut last_toast = Local::now();
                while running_clone.load(Ordering::Relaxed) {
                    thread::sleep(std::time::Duration::from_secs(RETRY_INTERVAL_SECS));

                    if check_defender_status() {
                        if disable_defender() {
                            log("Defender disabled successfully");
                            show_toast("Defender Disabler", "Defender has been disabled");
                            DEFENDER_DISABLED.store(true, Ordering::Relaxed);
                            break;
                        }
                    } else {
                        log("Defender is already disabled");
                        DEFENDER_DISABLED.store(true, Ordering::Relaxed);
                        break;
                    }

                    let now = Local::now();
                    let elapsed = now.signed_duration_since(last_toast);
                    if elapsed.num_seconds() >= TOAST_COOLDOWN_SECS {
                        show_toast(
                            "Defender Disabler",
                            "Failed to disable Defender - Please disable Tamper Protection",
                        );
                        last_toast = now;
                    }
                }
            }
        } else {
            log("Defender is already disabled");
            DEFENDER_DISABLED.store(true, Ordering::Relaxed);
        }
    });

    // Spawn registry monitor thread
    let running_clone = running.clone();
    let monitor_handle = thread::spawn(move || {
        monitor_registry_keys(running_clone);
    });

    // Spawn pipe handler thread
    let running_clone = running.clone();
    let pipe_handle = thread::spawn(move || {
        handle_pipe_commands(running_clone);
    });

    // Wait for termination signal
    while running.load(Ordering::Relaxed) {
        thread::sleep(std::time::Duration::from_millis(100));
    }

    log("Defender Disabler service shutting down");
    disable_handle.join().ok();
    monitor_handle.join().ok();
    pipe_handle.join().ok();
}

fn main() {
    let service_name = to_wide("DefenderDisabler\0");
    let service_table = [
        SERVICE_TABLE_ENTRYW {
            lpServiceName: service_name.as_ptr(),
            lpServiceProc: Some(service_main),
        },
        SERVICE_TABLE_ENTRYW {
            lpServiceName: ptr::null(),
            lpServiceProc: None,
        },
    ];

    unsafe {
        StartServiceCtrlDispatcherW(service_table.as_ptr());
    }
}

unsafe extern "system" fn def_window_proc(
    hwnd: HWND,
    msg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    DefWindowProcW(hwnd, msg, wparam, lparam)
}

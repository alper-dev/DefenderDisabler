#![windows_subsystem = "windows"]

use std::ptr;

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
type LPCWSTR = *const u16;

const ERROR_ALREADY_EXISTS: DWORD = 183;
const INVALID_HANDLE_VALUE: HANDLE = -1isize as HANDLE;
const GENERIC_READ: DWORD = 0x80000000;
const GENERIC_WRITE: DWORD = 0x40000000;
const OPEN_EXISTING: DWORD = 3;
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
const IDI_APPLICATION: LPCWSTR = 32512 as LPCWSTR;

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
    fn CreateMutexW(lpMutexAttributes: *mut std::ffi::c_void, bInitialOwner: BOOL, lpName: LPCWSTR) -> HANDLE;
    fn CloseHandle(hObject: HANDLE) -> BOOL;
    fn CreateFileW(lpFileName: LPCWSTR, dwDesiredAccess: DWORD, dwShareMode: DWORD, lpSecurityAttributes: *mut std::ffi::c_void, dwCreationDisposition: DWORD, dwFlagsAndAttributes: DWORD, hTemplateFile: HANDLE) -> HANDLE;
    fn ReadFile(hFile: HANDLE, lpBuffer: *mut std::ffi::c_void, nNumberOfBytesToRead: DWORD, lpNumberOfBytesRead: *mut DWORD, lpOverlapped: *mut std::ffi::c_void) -> BOOL;
    fn WriteFile(hFile: HANDLE, lpBuffer: *const std::ffi::c_void, nNumberOfBytesToWrite: DWORD, lpNumberOfBytesWritten: *mut DWORD, lpOverlapped: *mut std::ffi::c_void) -> BOOL;
}

#[link(name = "user32")]
extern "system" {
    fn RegisterClassExW(lpWndClass: *const WNDCLASSEXW) -> u16;
    fn CreateWindowExW(dwExStyle: DWORD, lpClassName: LPCWSTR, lpWindowName: LPCWSTR, dwStyle: DWORD, x: i32, y: i32, nWidth: i32, nHeight: i32, hWndParent: HWND, hMenu: HMENU, hInstance: HINSTANCE, lpParam: *mut std::ffi::c_void) -> HWND;
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

const WM_TRAYICON: u32 = WM_USER + 1;
const ID_STATUS: usize = 1;
const ID_VIEW_LOGS: usize = 2;
const ID_EXIT: usize = 3;

static mut TRAY_ICON: NOTIFYICONDATAW = NOTIFYICONDATAW {
    cbSize: std::mem::size_of::<NOTIFYICONDATAW>() as u32,
    hWnd: ptr::null_mut(),
    uID: 1,
    uFlags: NIF_ICON | NIF_MESSAGE | NIF_TIP,
    uCallbackMessage: WM_TRAYICON,
    hIcon: ptr::null_mut(),
    szTip: [0; 128],
    dwState: 0,
    dwStateMask: 0,
    szInfo: [0; 256],
    uVersion: 0,
    szInfoTitle: [0; 64],
    dwInfoFlags: 0,
    guidItem: [0; 16],
    hBalloonIcon: ptr::null_mut(),
};

fn to_wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

fn send_command(command: &str) -> String {
    unsafe {
        let pipe_name = to_wide(r"\\.\pipe\DefenderDisabler");
        let pipe = CreateFileW(
            pipe_name.as_ptr(),
            GENERIC_READ | GENERIC_WRITE,
            0,
            ptr::null_mut(),
            OPEN_EXISTING,
            0,
            ptr::null_mut(),
        );

        if pipe == INVALID_HANDLE_VALUE {
            return String::new();
        }

        let cmd = command.as_bytes();
        let mut bytes_written = 0;
        WriteFile(pipe, cmd.as_ptr() as *const _, cmd.len() as u32, &mut bytes_written, ptr::null_mut());

        let mut buffer = [0u8; 256];
        let mut bytes_read = 0;
        let result = ReadFile(pipe, buffer.as_mut_ptr() as *mut _, buffer.len() as u32, &mut bytes_read, ptr::null_mut());

        CloseHandle(pipe);

        if result != 0 && bytes_read > 0 {
            String::from_utf8_lossy(&buffer[..bytes_read as usize]).to_string()
        } else {
            String::new()
        }
    }
}

fn update_tooltip(status: &str) {
    unsafe {
        let tip = format!("Defender Status: {}\0", status);
        let tip_w = to_wide(&tip);
        TRAY_ICON.szTip[..tip_w.len().min(127)].copy_from_slice(&tip_w[..tip_w.len().min(127)]);
        Shell_NotifyIconW(NIM_MODIFY, &TRAY_ICON);
    }
}

fn show_context_menu(hwnd: HWND) {
    unsafe {
        let status = send_command("STATUS");
        let status_text = if status.trim() == "DISABLED" {
            "Defender Status: OFF\0"
        } else {
            "Defender Status: ON\0"
        };

        let menu = CreatePopupMenu();

        // Status item (disabled, just for display)
        let status_w = to_wide(status_text);
        AppendMenuW(menu, MF_STRING | MF_DISABLED | MF_GRAYED, ID_STATUS, status_w.as_ptr());

        AppendMenuW(menu, MF_SEPARATOR, 0, ptr::null());

        // View Logs
        let view_logs_w = to_wide("View Logs\0");
        AppendMenuW(menu, MF_STRING, ID_VIEW_LOGS, view_logs_w.as_ptr());

        AppendMenuW(menu, MF_SEPARATOR, 0, ptr::null());

        // Exit
        let exit_w = to_wide("Exit\0");
        AppendMenuW(menu, MF_STRING, ID_EXIT, exit_w.as_ptr());

        let mut point = POINT { x: 0, y: 0 };
        GetCursorPos(&mut point);

        SetForegroundWindow(hwnd);
        TrackPopupMenu(menu, TPM_BOTTOMALIGN | TPM_LEFTALIGN, point.x, point.y, 0, hwnd, ptr::null());

        DestroyMenu(menu);
    }
}

fn open_log_folder() {
    unsafe {
        let log_path = r"C:\Program Files\Defender Disabler\defender-disabler.log";
        let params = format!("/select,\"{}\"", log_path);

        let app_w = to_wide("explorer.exe\0");
        let params_w = to_wide(&params);

        ShellExecuteW(
            ptr::null_mut(),
            to_wide("open\0").as_ptr(),
            app_w.as_ptr(),
            params_w.as_ptr(),
            ptr::null(),
            SW_SHOW,
        );
    }
}

unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_TRAYICON => {
            match lparam as u32 {
                WM_RBUTTONUP | WM_CONTEXTMENU => {
                    show_context_menu(hwnd);
                    0
                }
                WM_LBUTTONUP => {
                    let status = send_command("STATUS");
                    update_tooltip(if status.trim() == "DISABLED" { "OFF" } else { "ON" });
                    0
                }
                _ => DefWindowProcW(hwnd, msg, wparam, lparam),
            }
        }
        WM_COMMAND => {
            let id = wparam & 0xFFFF;
            match id {
                ID_STATUS => 0,
                ID_VIEW_LOGS => {
                    open_log_folder();
                    0
                }
                ID_EXIT => {
                    send_command("EXIT");
                    Shell_NotifyIconW(NIM_DELETE, &TRAY_ICON);
                    PostQuitMessage(0);
                    0
                }
                _ => DefWindowProcW(hwnd, msg, wparam, lparam),
            }
        }
        WM_DESTROY => {
            Shell_NotifyIconW(NIM_DELETE, &TRAY_ICON);
            PostQuitMessage(0);
            0
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

fn main() {
    unsafe {
        // Single instance check
        let mutex_name = to_wide("Global\\DefenderDisablerTrayMutex\0");
        let mutex = CreateMutexW(ptr::null_mut(), 1, mutex_name.as_ptr());
        if mutex.is_null() {
            return;
        }
        if GetLastError() == ERROR_ALREADY_EXISTS {
            CloseHandle(mutex);
            return;
        }

        let class_name = to_wide("DefenderDisablerTray\0");

        let wc = WNDCLASSEXW {
            cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
            style: 0,
            lpfnWndProc: Some(window_proc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: GetModuleHandleW(ptr::null()),
            hIcon: LoadIconW(ptr::null_mut(), IDI_APPLICATION),
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
            to_wide("Defender Disabler\0").as_ptr(),
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

        // Set up tray icon
        TRAY_ICON.hWnd = hwnd;
        TRAY_ICON.hIcon = LoadIconW(ptr::null_mut(), IDI_APPLICATION);

        let tip = to_wide("Defender Disabler\0");
        TRAY_ICON.szTip[..tip.len().min(127)].copy_from_slice(&tip[..tip.len().min(127)]);

        Shell_NotifyIconW(NIM_ADD, &TRAY_ICON);

        // Initial status check
        let status = send_command("STATUS");
        update_tooltip(if status.trim() == "DISABLED" { "OFF" } else { "ON" });

        // Message loop
        let mut msg: MSG = std::mem::zeroed();
        while GetMessageW(&mut msg, ptr::null_mut(), 0, 0) > 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}

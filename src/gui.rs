use super::*;
use windows::Win32::UI::WindowsAndMessaging::*;

pub async fn run_ui() -> Result<()> {
    unsafe {
        let instance = GetModuleHandleA(None)?;
        let window_class = s!("UniversalDriverWindow");
        
        let wc = WNDCLASSA {
            hInstance: instance,
            lpszClassName: window_class,
            lpfnWndProc: Some(wndproc),
            ..Default::default()
        };
        
        RegisterClassA(&wc);
        
        CreateWindowExA(
            0,
            window_class,
            s!("Universal Printer Driver"),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            800,
            600,
            None,
            None,
            instance,
            None,
        );
        
        let mut msg = MSG::default();
        while GetMessageA(&mut msg, None, 0, 0).into() {
            TranslateMessage(&msg);
            DispatchMessageA(&msg);
        }
    }
    Ok(())
}

extern "system" fn wndproc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_CREATE => {
            unsafe {
                CreateWindowA(
                    s!("BUTTON"),
                    s!("Scan Printers"),
                    WS_VISIBLE | WS_CHILD | BS_DEFPUSHBUTTON,
                    10,
                    10,
                    120,
                    30,
                    hwnd,
                    HMENU(1),
                    GetModuleHandleA(None).unwrap(),
                    None,
                );
            }
            LRESULT(0)
        }
        WM_COMMAND => {
            LRESULT(0)
        }
        WM_DESTROY => {
            PostQuitMessage(0);
            LRESULT(0)
        }
        _ => DefWindowProcA(hwnd, msg, wparam, lparam),
    }
}
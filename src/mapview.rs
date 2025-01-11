use std::sync::Once;

use windows::{
    core::{w, Result, HRESULT, HSTRING},
    Win32::{
        Foundation::{COLORREF, HWND, LPARAM, LRESULT, WPARAM},
        Graphics::Gdi::{CreateSolidBrush, GetSysColor, COLOR_WINDOW},
        System::LibraryLoader::GetModuleHandleW,
        UI::WindowsAndMessaging::{
            CreateWindowExW, DefWindowProcW, GetWindowLongPtrA, LoadCursorW, RegisterClassW,
            SetWindowLongPtrA, CREATESTRUCTA, CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, GWLP_USERDATA,
            IDC_ARROW, WINDOW_EX_STYLE, WM_CREATE, WNDCLASSW, WS_CHILD, WS_HSCROLL, WS_VISIBLE,
            WS_VSCROLL,
        },
    },
};

use crate::map::Map;

const WINDOW_NAME: &str = "map_view";

static REGISTER_WINDOW_CLASS: Once = Once::new();

pub struct MapView {
    handle: Option<HWND>,
    map: Option<Map>,
}

impl MapView {
    pub fn new(parent: HWND) -> Result<Box<Self>> {
        let instance = unsafe { GetModuleHandleW(None)? };
        let window_classname = w!("bytetrail.window.mapview");
        let background = unsafe { GetSysColor(COLOR_WINDOW) };

        REGISTER_WINDOW_CLASS.call_once(|| {
            // use defaults for all other fields
            let class = WNDCLASSW {
                lpfnWndProc: Some(Self::wnd_proc),
                hbrBackground: unsafe { CreateSolidBrush(COLORREF(background)) },
                hInstance: instance.into(),
                style: CS_HREDRAW | CS_VREDRAW,
                hCursor: unsafe { LoadCursorW(None, IDC_ARROW).ok().unwrap() },
                lpszClassName: window_classname,
                ..Default::default()
            };
            assert_ne!(unsafe { RegisterClassW(&class) }, 0);
        });

        let mut map_view = Box::new(MapView {
            handle: None,
            map: None,
        });
        // create the window using Self reference
        let result = unsafe {
            CreateWindowExW(
                WINDOW_EX_STYLE::default(),
                window_classname,
                &HSTRING::from(WINDOW_NAME),
                WS_VISIBLE | WS_CHILD | WS_HSCROLL | WS_VSCROLL,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                400,
                300,
                Some(parent),
                None,
                Some(instance.into()),
                Some(map_view.as_mut() as *mut _ as _),
            )
        };
        match result {
            Ok(_window) => {}
            Err(e) => {
                // HRESULT(0) means the window was created but the message loop will not run
                if e.code() == HRESULT(0) {
                    println!("Window created but message loop will not run");
                } else {
                    return Err(e);
                }
            }
        }
        Ok(map_view)
    }

    fn message_handler(&mut self, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        match message {
            _ => unsafe { DefWindowProcW(self.handle.unwrap(), message, wparam, lparam) },
        }
    }

    unsafe extern "system" fn wnd_proc(
        window: HWND,
        message: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        if message == WM_CREATE {
            let create_struct = lparam.0 as *const CREATESTRUCTA;
            let this = (*create_struct).lpCreateParams as *mut Self;
            (*this).handle = Some(window);
            SetWindowLongPtrA(window, GWLP_USERDATA, this as _);
        } else {
            let this = GetWindowLongPtrA(window, GWLP_USERDATA) as *mut Self;

            if !this.is_null() {
                return (*this).message_handler(message, wparam, lparam);
            }
        }
        DefWindowProcW(window, message, wparam, lparam)
    }
}

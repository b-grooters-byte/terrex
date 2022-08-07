use std::sync::Once;

use windows::{
    core::{Result, HSTRING},
    w,
    Win32::{
        Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, WPARAM},
        Graphics::Gdi::CreateSolidBrush,
        System::LibraryLoader::GetModuleHandleW,
        UI::WindowsAndMessaging::{
            CreateWindowExW, LoadCursorW, RegisterClassW, COLOR_WINDOW, CS_HREDRAW, CS_VREDRAW,
            CW_USEDEFAULT, HMENU, IDC_ARROW, WINDOW_EX_STYLE, WNDCLASSW, WS_CHILD, WS_HSCROLL,
            WS_VISIBLE, WS_VSCROLL,
        },
    },
};

use crate::map::Map;

const WINDOW_NAME: &str = "map_view";

static REGISTER_WINDOW_CLASS: Once = Once::new();
static WINDOW_CLASS_NAME: &HSTRING = w!("bytetrail.window.mapview");

struct MapView {
    handle: HWND,
    map: Option<Map>,
}

impl MapView {
    pub fn new() -> Result<Box<Self>> {
        let instance = unsafe { GetModuleHandleW(None)? };

        REGISTER_WINDOW_CLASS.call_once(|| {
            // use defaults for all other fields
            let class = WNDCLASSW {
                lpfnWndProc: Some(Self::wnd_proc),
                hbrBackground: unsafe { CreateSolidBrush(COLOR_WINDOW.0) },
                hInstance: instance,
                style: CS_HREDRAW | CS_VREDRAW,
                hCursor: unsafe { LoadCursorW(HINSTANCE(0), IDC_ARROW).ok().unwrap() },
                lpszClassName: WINDOW_CLASS_NAME.into(),
                ..Default::default()
            };
            assert_ne!(unsafe { RegisterClassW(&class) }, 0);
        });

        let mut map_view = Box::new(MapView {
            handle: HWND(0),
            map: None,
        });
        // create the window using Self reference
        let window = unsafe {
            CreateWindowExW(
                WINDOW_EX_STYLE::default(),
                WINDOW_CLASS_NAME,
                &HSTRING::from(WINDOW_NAME),
                WS_VISIBLE | WS_CHILD | WS_HSCROLL | WS_VSCROLL,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                400,
                300,
                HWND(0),
                HMENU(0),
                instance,
                map_view.as_mut() as *mut _ as _,
            )
        };
        Ok(map_view)
    }

    unsafe extern "system" fn wnd_proc(
        window: HWND,
        message: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        LRESULT(0)
    }
}

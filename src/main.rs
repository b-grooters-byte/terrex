use mapview::MapView;
use std::sync::Once;
use windows::{
    core::{w, Result, HSTRING},
    Win32::{
        Foundation::{COLORREF, HWND, LPARAM, LRESULT, WPARAM},
        Graphics::{
            Direct2D::ID2D1Factory1,
            Gdi::{CreateSolidBrush, GetSysColor, COLOR_WINDOW, HBRUSH},
        },
        System::{
            Com::{CoInitializeEx, COINIT_MULTITHREADED},
            LibraryLoader::GetModuleHandleW,
        },
        UI::WindowsAndMessaging::{
            CreateWindowExW, DefWindowProcW, DispatchMessageW, GetMessageW, GetWindowLongPtrA,
            LoadCursorW, RegisterClassW, SetWindowLongPtrA, ShowWindow, CREATESTRUCTA, CS_HREDRAW,
            CS_VREDRAW, CW_USEDEFAULT, GWLP_USERDATA, IDC_ARROW, MSG, SW_SHOW, WINDOW_EX_STYLE,
            WM_CREATE, WNDCLASSW, WS_OVERLAPPEDWINDOW, WS_VISIBLE,
        },
    },
};

mod direct2d;
mod map;
mod mapview;

static REGISTER_WINDOW_CLASS: Once = Once::new();

fn main() -> Result<()> {
    unsafe {
        let result = CoInitializeEx(None, COINIT_MULTITHREADED);
        if result.is_err() {
            return Err(result.into());
        }
    }
    let factory = direct2d::create_factory()?;
    let _m = AppWindow::new("MineSweeper", &factory);
    let mut message = MSG::default();
    unsafe {
        while GetMessageW(&mut message, None, 0, 0).into() {
            DispatchMessageW(&message);
        }
    }
    Ok(())
}

pub(crate) struct AppWindow<'a> {
    handle: Option<HWND>,
    map_view: Option<Box<MapView>>,
    factory: &'a ID2D1Factory1,
}

impl<'a> AppWindow<'a> {
    pub(crate) fn new(title: &'static str, factory: &'a ID2D1Factory1) -> Result<Box<Self>> {
        let instance = unsafe { GetModuleHandleW(None)? };
        let window_classname = w!("bytetrail.window.terrex");

        // synchronization for a one time initialization of FFI call
        REGISTER_WINDOW_CLASS.call_once(|| {
            // use defaults for all other fields
            let class = WNDCLASSW {
                lpfnWndProc: Some(Self::wnd_proc),
                hbrBackground: unsafe { CreateSolidBrush(COLORREF(GetSysColor(COLOR_WINDOW))) },
                hInstance: instance.into(),
                style: CS_HREDRAW | CS_VREDRAW,
                hCursor: unsafe { LoadCursorW(None, IDC_ARROW).ok().unwrap() },
                lpszClassName: window_classname,
                ..Default::default()
            };
            assert_ne!(unsafe { RegisterClassW(&class) }, 0);
        });
        let mut app_window = Box::new(AppWindow {
            handle: None,
            map_view: None,
            factory,
        });
        // create the window using Self reference
        let window = unsafe {
            CreateWindowExW(
                WINDOW_EX_STYLE::default(),
                window_classname,
                &HSTRING::from(title),
                WS_VISIBLE | WS_OVERLAPPEDWINDOW,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                400,
                300,
                None,
                None,
                Some(instance.into()),
                Some(app_window.as_mut() as *mut _ as _),
            )
        }?;
        unsafe { ShowWindow(window, SW_SHOW) };

        Ok(app_window)
    }

    fn message_handler(
        &mut self,
        window: HWND,
        message: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        match message {
            WM_CREATE => match MapView::new(self.handle.unwrap()) {
                Ok(map_view) => {
                    self.map_view = Some(map_view);
                    LRESULT(0)
                }
                Err(e) => {
                    eprintln!("Failed to create MapView: {:?}", e);
                    LRESULT(-1)
                }
            },
            _ => unsafe { DefWindowProcW(window, message, wparam, lparam) },
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
        }
        let this = GetWindowLongPtrA(window, GWLP_USERDATA) as *mut Self;

        if !this.is_null() {
            return (*this).message_handler(window, message, wparam, lparam);
        }
        DefWindowProcW(window, message, wparam, lparam)
    }
}

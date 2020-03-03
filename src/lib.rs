use failure::{Error, Fail};
use std::ffi::c_void;
use widestring::WideCString;
use win_toast_sys::*;
pub use win_toast_sys::{
  IWinToastHandler_WinToastDismissalReason, WinToastTemplate_TextField,
  WinToastTemplate_WinToastTemplateType,
};

pub type ToastId = INT64;

#[derive(Debug, Fail)]
enum WinToastError {
  #[fail(display = "your system in not supported")]
  Incompatible,

  #[fail(display = "WinToast error {:?}", error)]
  WinToastError { error: WinToast_WinToastError },
}

pub struct WinToast {
  app_name: WideCString,
  aumi: WideCString,

  inner: *mut c_void,
}
impl WinToast {
  pub fn initialize(app_name: &str, company_name: &str, product_name: &str) -> Result<Self, Error> {
    let aumi = format!("{}.{}", company_name, product_name);

    let win_toast = Self {
      app_name: WideCString::from_str(app_name)?,
      aumi: WideCString::from_str(aumi)?,

      inner: unsafe { WinToast_instance() },
    };

    unsafe {
      if !WinToast_isCompatible() {
        return Err(WinToastError::Incompatible.into());
      }

      WinToast_setAppName(win_toast.inner, win_toast.app_name.as_ptr());

      WinToast_setAppUserModelId(win_toast.inner, win_toast.aumi.as_ptr());

      let error = WinToast_initialize(win_toast.inner);

      if error != WinToast_WinToastError::NoError {
        return Err(WinToastError::WinToastError { error }.into());
      }
    }

    Ok(win_toast)
  }

  pub fn show_toast(
    &mut self,
    template: &WinToastTemplate,
    handler: &WinToastHandler,
  ) -> Result<ToastId, Error> {
    let ret = unsafe { WinToast_showToast(self.inner, template.inner, handler.inner) };

    if ret.error != WinToast_WinToastError::NoError {
      return Err(WinToastError::WinToastError { error: ret.error }.into());
    }

    Ok(ret.id)
  }
}
// I keep getting (exit code: 0xc0000374, STATUS_HEAP_CORRUPTION)
// when trying to use "new", on delete
// impl Drop for WinToast {
//   fn drop(&mut self) {
//     println!("drop toast");
//     unsafe {
//       WinToast_delete(self.inner);
//     }
//   }
// }

pub struct WinToastTemplate {
  inner: *mut c_void,
  c_strings: Vec<WideCString>,
}
impl WinToastTemplate {
  pub fn new(template_type: WinToastTemplate_WinToastTemplateType) -> Self {
    let inner = unsafe { WinToastTemplate_new(template_type) };

    Self {
      inner,
      c_strings: Vec::new(),
    }
  }

  pub fn set_text_field(
    &mut self,
    text: &str,
    field: WinToastTemplate_TextField,
  ) -> Result<(), Error> {
    let text = WideCString::from_str(text)?;

    unsafe {
      WinToastTemplate_setTextField(self.inner, text.as_ptr(), field);
    }

    // keep it alive
    self.c_strings.push(text);

    Ok(())
  }
}
impl Drop for WinToastTemplate {
  fn drop(&mut self) {
    unsafe {
      WinToastTemplate_delete(self.inner);
    }
  }
}

pub struct WinToastHandler {
  inner: *mut c_void,
}
impl WinToastHandler {
  pub fn new(
    activated_callback: extern "C" fn(actionIndex: ::std::os::raw::c_int),
    dismissed_callback: extern "C" fn(state: IWinToastHandler_WinToastDismissalReason),
    failed_callback: extern "C" fn(),
  ) -> Self {
    let inner = unsafe {
      WinToastHandler_new(
        Some(activated_callback),
        Some(dismissed_callback),
        Some(failed_callback),
      )
    };

    Self { inner }
  }
}
impl Drop for WinToastHandler {
  fn drop(&mut self) {
    unsafe {
      WinToastHandler_delete(self.inner);
    }
  }
}

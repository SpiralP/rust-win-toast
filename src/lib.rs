use failure::{Error, Fail};
use std::ffi::c_void;
use widestring::WideCString;
use win_toast_sys::*;

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

      inner: unsafe { WinToast_new() },
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
    template: WinToastTemplate,
    handler: WinToastHandler,
  ) -> Result<ToastId, Error> {
    let ret = unsafe { WinToast_showToast(self.inner, template.inner, handler.inner) };

    if ret.error != WinToast_WinToastError::NoError {
      return Err(WinToastError::WinToastError { error: ret.error }.into());
    }

    Ok(ret.id)
  }
}
impl Drop for WinToast {
  fn drop(&mut self) {
    unsafe {
      WinToast_delete(self.inner);
    }
  }
}

pub struct WinToastTemplate {
  inner: *mut c_void,
}
impl WinToastTemplate {
  pub fn new(template_type: WinToastTemplate_WinToastTemplateType) -> Self {
    let inner = unsafe { WinToastTemplate_new(template_type) };

    Self { inner }
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
    activated_callback: HandlerToastActivatedCallback,
    dismissed_callback: HandlerToastDismissedCallback,
    failed_callback: HandlerToastFailedCallback,
  ) -> Self {
    let inner =
      unsafe { WinToastHandler_new(activated_callback, dismissed_callback, failed_callback) };

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

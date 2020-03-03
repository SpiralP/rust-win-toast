use win_toast::*;

#[test]
fn it_works() {
  use lazy_static::lazy_static;
  use std::{
    sync::{Condvar, Mutex},
    time::Duration,
  };

  lazy_static! {
    static ref PAIR: (Mutex<()>, Condvar) = (Mutex::new(()), Condvar::new());
  }

  fn wake() {
    let (lock, cvar) = &*PAIR;
    let _guard = lock.lock().unwrap();
    cvar.notify_all();
  }

  fn wait() {
    let (lock, cvar) = &*PAIR;
    let guard = lock.lock().unwrap();
    cvar.notify_all();

    // 30 seconds probably long enough for only buggy toasts
    let _guard = cvar.wait_timeout(guard, Duration::from_secs(30)).unwrap();
  }

  extern "C" fn activated(action_index: i32) {
    println!("activated {:?}", action_index);

    wake();
  }

  extern "C" fn dismissed(state: IWinToastHandler_WinToastDismissalReason) {
    println!("dismissed {:?}", state);

    wake();
  }

  extern "C" fn failed() {
    println!("failed");

    wake();
  }

  let mut win_toast = WinToast::initialize("Hello World", "Author", "").unwrap();

  let mut template = WinToastTemplate::new(WinToastTemplate_WinToastTemplateType::Text02);

  template
    .set_text_field("first line", WinToastTemplate_TextField::FirstLine)
    .unwrap();

  template
    .set_text_field("second line", WinToastTemplate_TextField::SecondLine)
    .unwrap();

  let handler = WinToastHandler::new(activated, dismissed, failed);

  win_toast.show_toast(&template, &handler).unwrap();

  wait();
}

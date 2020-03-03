use win_toast::*;

#[test]
fn it_works() {
  extern "C" fn activated(action_index: i32) {
    println!("activated {:?}", action_index);
  }

  extern "C" fn dismissed(state: IWinToastHandler_WinToastDismissalReason) {
    println!("dismissed {:?}", state);
  }

  extern "C" fn failed() {
    println!("failed");
  }

  let mut win_toast = WinToast::initialize("Test App", "Company", "App Name").unwrap();

  let mut template = WinToastTemplate::new(WinToastTemplate_WinToastTemplateType::Text01);

  template
    .set_text_field("hello!", WinToastTemplate_TextField::FirstLine)
    .unwrap();

  let handler = WinToastHandler::new(activated, dismissed, failed);

  win_toast.show_toast(&template, &handler).unwrap();

  // don't exit program so quickly!
  std::thread::sleep(std::time::Duration::from_millis(500));
}

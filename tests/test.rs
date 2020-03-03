use win_toast::*;

#[test]
fn it_works() {
  ag();
}

fn ag() {
  // std::thread::spawn(move || {
  //   std::thread::sleep(std::time::Duration::from_secs(30));
  //   let _ = WAIT.0.send(());
  // });

  extern "C" fn activated(action_index: i32) {
    println!("activated {:?}", action_index);
    // let _ = WAIT.0.send(());
  }

  extern "C" fn dismissed(state: IWinToastHandler_WinToastDismissalReason) {
    println!("dismissed {:?}", state);
    // let _ = WAIT.0.send(());
  }

  extern "C" fn failed() {
    println!("failed");
    // let _ = WAIT.0.send(());
  }

  let mut win_toast = WinToast::initialize("Test App", "Company", "App Name").unwrap();

  let mut template = WinToastTemplate::new(WinToastTemplate_WinToastTemplateType::Text01);

  template
    .set_text_field("hello!", WinToastTemplate_TextField::FirstLine)
    .unwrap();

  let handler = WinToastHandler::new(activated, dismissed, failed);

  win_toast.show_toast(&template, &handler).unwrap();

  println!("end");
}

use win_toast::WinToast;

#[test]
fn it_works() {
  WinToast::initialize("Test App", "Company", "App Name").unwrap();
}

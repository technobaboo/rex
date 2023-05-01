use dialog::DialogBox;

pub trait ExpectDialog<T> {
    fn expect_dialog(self, msg: &str) -> T;
}

impl<T, E: std::fmt::Debug> ExpectDialog<T> for Result<T, E> {
    fn expect_dialog(self, msg: &str) -> T {
        match self {
            Ok(value) => return value,
            Err(_) => {
                let mut backend = dialog::backends::Zenity::new(); //TODO: fork to switch to https://crates.io/crates/native-dialog as dialog does not interact nicely with KDE. native-dialog is a little bugged but the fix seems easy
                backend.set_width(200);
                backend.set_height(10);
                backend.set_icon("error");
                dialog::Message::new(msg)
                    .title("Fatal Error")
                    .show_with(backend)
                    .expect("Could not display dialog box");
                panic!("{}", msg)
            }
        }
    }
}

impl<T> ExpectDialog<T> for Option<T> {
    fn expect_dialog(self, msg: &str) -> T {
        match self {
            Some(value) => return value,
            None => {
                let mut backend = dialog::backends::Zenity::new();
                backend.set_width(200);
                backend.set_height(10);
                backend.set_icon("error");
                dialog::Message::new(msg)
                    .title("Fatal Error")
                    .show_with(backend)
                    .expect("Could not display dialog box");
                panic!("{}", msg)
            }
        }
    }
}

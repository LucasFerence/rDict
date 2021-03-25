use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;

pub fn write(val: String) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(val).unwrap();
}
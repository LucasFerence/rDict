use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;

use crate::Res;

pub fn write(val: String) -> Res<()> {

    let mut ctx: ClipboardContext = ClipboardProvider::new()?;
    ctx.set_contents(val)?;

    Ok(())
}
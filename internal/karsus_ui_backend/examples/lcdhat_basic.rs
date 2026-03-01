use lcdhat::{Config, Font, Key, LcdHat, Result, color};

fn main() -> Result<()> {
    let mut lcd = LcdHat::new(Config::default())?;

    lcd.clear(color::WHITE)?;
    lcd.draw_text(
        8,
        8,
        "lcdhat rust",
        Font::Font12,
        color::BLACK,
        color::WHITE,
    )?;
    lcd.present()?;

    loop {
        match lcd.poll_key()? {
            Some(event) => {
                if event.pressed {
                    match event.key {
                        Key::Press => break,
                        Key::K1 => lcd.clear(color::RED)?,
                        Key::K2 => lcd.clear(color::GREEN)?,
                        Key::K3 => lcd.clear(color::BLUE)?,
                        _ => {}
                    }
                    lcd.present()?;
                }
            }
            None => LcdHat::sleep_ms(10),
        }
    }

    Ok(())
}

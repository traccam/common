
use embedded_graphics::mono_font::MonoTextStyle;
use core::fmt::Debug;
use embedded_graphics::text::Baseline;
use embedded_graphics::prelude::Point;
use embedded_graphics::text::Text;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::mono_font::ascii::FONT_6X10;
use embedded_graphics::prelude::DrawTarget;
use chrono::NaiveTime;
use embedded_graphics::Drawable;
use embedded_graphics::mono_font::MonoTextStyleBuilder;


#[derive(Clone, Default)]
pub struct DisplayState {
	pub time: NaiveTime,
	pub lat: f64,
	pub lon: f64,
	pub sats: u8,
}

pub fn draw_status_display<D>(display: &mut D, state: &DisplayState)
	where
	D: DrawTarget<Color = BinaryColor>,
	D::Error: Debug
{
	const TXT_STYLE: MonoTextStyle<BinaryColor> = MonoTextStyleBuilder::new()
		.font(&FONT_6X10)
		.text_color(BinaryColor::On)
		.build();

	display.clear(BinaryColor::Off).unwrap();

	Text::with_baseline(&heapless::format!(15; "{} {}", state.time, state.sats).unwrap(), Point::zero(), TXT_STYLE, Baseline::Top)
		.draw(display)
		.unwrap();

	Text::with_baseline(&heapless::format!(10; "N{:.5}", state.lat).unwrap(), Point::new(0, 9), TXT_STYLE, Baseline::Top)
		.draw(display)
		.unwrap();

	Text::with_baseline(&heapless::format!(10; "E{:.5}", state.lon).unwrap(), Point::new(0, 18), TXT_STYLE, Baseline::Top)
		.draw(display)
		.unwrap();
}
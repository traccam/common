
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

	Text::with_baseline(&heapless::format!(30; "{} {}", state.time, state.sats).unwrap(), Point::zero(), TXT_STYLE, Baseline::Top)
		.draw(display)
		.unwrap();

	Text::with_baseline(&heapless::format!(10; "N{:>8.5}", state.lat).unwrap(), Point::new(0, 9), TXT_STYLE, Baseline::Top)
		.draw(display)
		.unwrap();

	Text::with_baseline(&heapless::format!(10; "E{:>8.5}", state.lon).unwrap(), Point::new(0, 18), TXT_STYLE, Baseline::Top)
		.draw(display)
		.unwrap();
}

#[cfg(feature = "simulated_data")]
pub mod simulator {
	use chrono::Local;
	use crate::display::DisplayState;

	#[derive(Default)]
	pub struct StateSimulator {
		rng: fastrand::Rng,
	}

	impl StateSimulator {
		pub fn gen_next(&mut self) -> DisplayState {
			DisplayState {
				time: Local::now().naive_local().time(),
				lat: self.rng.f64() * 60.0,
				lon: self.rng.f64() * 60.0,
				sats: self.rng.u8(0..99),
			}
		}
	}
}
use app::Laurel;

pub mod app;
pub mod ui;

fn main() -> iced::Result {
    iced::application("Laurel", Laurel::update, Laurel::view)
        .settings(Laurel::settings())
        .scale_factor(|_| 1.6)
        .run_with(Laurel::new)?;

    Ok(())
}

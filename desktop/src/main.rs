mod app;
mod local;

use tracing_subscriber::EnvFilter;

const APPID: &str = "com.fyralabs.MAZ";

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let filter = EnvFilter::try_from_env("MAZ_LOG").unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_ansi(true)
        .pretty()
        .init();
    tracing::trace!("Q.E.D. ■");
    tracing::info!("M.A.Z. {}", env!("CARGO_PKG_VERSION"));

    let app = libhelium::Application::builder()
        .application_id(APPID)
        .flags(libhelium::gio::ApplicationFlags::default())
        .build();

    relm4::RelmApp::from_app(app).run::<app::AppModel>(());

    Ok(())
}

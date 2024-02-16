use anyhow::Result;

use app::App;
use commands::Options;

fn main() -> Result<()> {
    let _app = App::new()?;

    Options::run()
}

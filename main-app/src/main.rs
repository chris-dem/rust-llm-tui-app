use color_eyre::Result as CResult;

use crate::state::App;

mod state;
mod ui;

async fn ollama_main() {
    // let ollama = Ollama::default();
    // let model = "deepseek-r1:latest".to_string();
    // let prompt = "This is a test".to_string();
    //
    // let res = ollama.generate(GenerationRequest::new(model, prompt)).await;
    // if let Ok(res) = dbg!(res) {
    //     println!("{}", res.response);
    // }
}

#[tokio::main]
async fn main() -> CResult<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal).await;
    ratatui::restore();
    app_result
}

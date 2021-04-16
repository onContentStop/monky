use dialoguer::console::style;
use dialoguer::console::Style;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Input;
use logos::Logos;
use monky::syntax::SyntaxKind;

fn main() {
    log4rs::init_file("monky-log.yml", Default::default()).unwrap();
    let theme = ColorfulTheme {
        prompt_prefix: style("monky".into()).yellow(),
        prompt_style: Style::new().dim(),
        prompt_suffix: style(">>".into()).bold(),
        success_prefix: style("monky".into()).green(),
        success_suffix: style(">>".into()).bold(),
        values_style: Style::new().blue().bright(),
        ..Default::default()
    };
    let mut input = Input::<String>::with_theme(&theme);
    input.with_prompt(":");

    loop {
        let text = input.interact().unwrap();
        if text == "/q" {
            break;
        }

        let lexer = SyntaxKind::lexer(text.as_str());

        for tok in lexer {
            println!("{:?}", tok);
        }
    }
}

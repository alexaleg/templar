use clap::{Parser, Subcommand};
mod reader;
mod writer;

#[derive(Parser)]
struct Cli {
    template_name: String,

    #[clap(default_value=".")]
    out_path: std::path::PathBuf,

    #[arg(short = 'f')]
    pprint: bool,
}

fn main() {
    let args = Cli::parse();
    let template_name = args.template_name;
    if args.pprint {
        reader::pprint(template_name);
        return;
    }
    let out_path = args.out_path;

    let template_lines = reader::get_template(template_name);

    writer::write_template(template_lines, &out_path);

    //println!("template: {}", template);
    //write_template(template);

    //Ok(())
}

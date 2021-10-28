use std::fs::read_to_string;
use std::path::PathBuf;
use structopt::StructOpt;
use cook_with_rust_parser::parse;

#[derive(Debug, StructOpt)]
#[structopt(name = "crab-soup-cli", about = "CLI for the CookLang description language.")]
struct Opt {
    #[structopt(subcommand)]
    operation: Operations,
}

#[derive(Debug, StructOpt)]
enum Operations {
    Recipe {
        #[structopt(short, parse(from_os_str))]
        source: PathBuf,
        #[structopt(subcommand)]
        operation: RecipeOpt
    }
}

#[derive(Debug, StructOpt)]
enum RecipeOpt {
    JSON {
        #[structopt(parse(from_os_str))]
        path: PathBuf,
    },
    Markdown {
        #[structopt(parse(from_os_str))]
        path: PathBuf,
    },
}

fn main() {
    let opt = Opt::from_args();
    match opt.operation {
        Operations::Recipe {
            operation,
            source
        } => {
            let inp_recipe = read_to_string(source)
                .expect("Reading input file failed");
            let recipe = parse(&inp_recipe)
                .expect("Error during parsing of input file");
            match operation {
                RecipeOpt::JSON { path } => {
                    let json = serde_json::to_string(&recipe)
                        .expect("Error serializing to string.");
                    std::fs::write(path, json)
                        .expect("Error during writing of JSON to file");
                }
                RecipeOpt::Markdown { path } => {
                    let md = cook_markdown::recipe_to_markdown(&recipe);
                    std::fs::write(path, md)
                        .expect("Error during writing of markdown to file");
                }
            }
        }
    }
}

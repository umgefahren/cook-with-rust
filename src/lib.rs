extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "../CookLang.pest"]
pub struct CookParser;


pub fn parse(inp: &str) {
    let successful_parse = CookParser::parse(Rule::cook_lang, inp).expect("Failing");
    println!("{:#?}", successful_parse);
}

#[cfg(test)]
mod tests {
    use crate::{CookParser, parse};
    use crate::pest::Parser;

    #[test]
    fn it_works() {
        let test_rec = String::from("\
>> value: key\n\
>> servings: 1|2|3|4\n\
Get some @fruit salat{1|2|4%kg}\n\
Use the #potato masher{}
"
        );

        parse(&test_rec);
    }
}

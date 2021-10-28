use std::ops::Not;
use cook_with_rust_parser::*;


pub fn recipe_to_markdown(recipe: &Recipe) -> String {
    let mut ingredient_specifier_iter = recipe.metadata.ingredients_specifiers.iter();
    let mut cookware_iter = recipe.metadata.cookware.iter();
    let mut timer_iter = recipe.metadata.timer.iter();
    let mut result_string = String::new();

    if recipe.metadata.ingredients.is_empty().not() {
        result_string += "### Ingredients\n";
        result_string += "| Ingredient | Amount | Unit |\n";
        result_string += "|------------|--------|------|\n";

        let ingredients = recipe.metadata.ingredients.values();
        ingredients.for_each(|e| {
            let amount = match &e.amount {
                Some(d) => {
                    match d {
                        Amount::Multi(dd) => {
                            format!("servings * {}", dd)
                        }
                        Amount::Servings(dd) => {
                            let mut servings = String::new();
                            dd.iter().for_each(|a| {
                                servings.push_str(&a.to_string());
                                servings.push('|');
                            });
                            servings.pop();
                            servings
                        }
                        Amount::Single(dd) => {
                            dd.to_string()
                        }
                    }
                },
                None => "-".to_string(),
            };
            let unit = match &e.unit {
                None => "-".to_string(),
                Some(u) => u.to_string(),
            };
            let line = format!("| {} | {} | {} |\n", e.name, amount, unit);
            result_string += &line;
        });
        result_string += "\n"
    }

    if recipe.metadata.cookware.is_empty().not() {
        result_string += "\n### Cookware:\n";
        recipe.metadata.cookware.iter().for_each(|cookw| {
            result_string += format!("* {}\n", cookw).as_str();
        });
    }

    recipe.instruction.chars().for_each(|char| {
        if char == '@' {
            let insert_ingredient = ingredient_specifier_iter.next().unwrap();
            let ingredient_referenced = recipe.metadata.ingredients.get(&insert_ingredient.ingredient).unwrap();
            let ingredient_name = &ingredient_referenced.name;
            let ingredient_unit = match &ingredient_referenced.unit {
                None => "".to_string(),
                Some(d) => {
                    let mut res = String::from(' ');
                    res.push_str(d.as_str());
                    res
                },
            };
            let ingredient_amount = match &insert_ingredient.amount_in_step {
                Amount::Multi(d) => {
                    format!(" *{}", d)
                }
                Amount::Servings(dd) => {
                    let mut servings = String::new();
                    servings.push(' ');
                    dd.iter().for_each(|a| {
                        servings.push_str(&a.to_string());
                        servings.push('|');
                    });
                    servings.pop();
                    // servings.push(' ');
                    servings
                }
                Amount::Single(d) => {
                    let mut res = String::from(' ');
                    res.push_str(d.to_string().as_str());
                    // res.push(' ');
                    res
                }
            };
            let insert_string = format!("__{}{}{}__", ingredient_name, ingredient_amount, ingredient_unit);
            result_string += &insert_string;
        } else if char == '#' {
            result_string.push_str("_");
            result_string.push_str(cookware_iter.next().unwrap());
            result_string.push_str("_");
        } else if char == '~' {
            let timer = timer_iter.next().unwrap();
            result_string.push_str("__");
            result_string.push_str(&timer.amount.to_string());
            result_string.push(' ');
            result_string.push_str(&timer.unit);
            result_string.push_str("__");
        } else {
            result_string.push(char);
        }
    });

    result_string
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use cook_with_rust_parser::parse;
    use crate::recipe_to_markdown;

    #[test]
    fn it_works() {
        let inp = read_to_string("../spec/examples/Coffee Souffle.cook").unwrap();
        let recipe = parse(&inp).unwrap();

        let _result = recipe_to_markdown(&recipe);

        // std::fs::write("TEST.md", result);
    }
}

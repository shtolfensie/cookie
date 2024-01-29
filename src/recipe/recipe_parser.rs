use leptos::{IntoView, template, view};
use nom::{Parser};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, anychar, multispace0, digit1, newline};
use nom::combinator::{peek, recognize, eof};
use nom::error::ParseError;
use nom::multi::{many_till, many1};
use nom::sequence::{delimited, preceded};
use nom::IResult;

use anyhow::Result;


pub fn parse(input: &str) -> Result<Vec<Recipe>> {
    let (rest, _foreword) = md_text(input).map_err(|e| e.to_owned())?;
    let mut rest = rest;

    let mut recipes: Vec<Recipe> = Vec::new();

    loop {
        let (rest_name, recipe_name) = match ordered_list_item(rest).map_err(|e| e.to_owned()) {
            Ok(r) => r,
            Err(_) => break,
        };

        let (rest_body, recipe_body) = match unordered_list(rest_name).map_err(|e| e.to_owned()) {
            Ok(r) => r,
            Err(_) => break,
        };

        recipes.push(Recipe { name: recipe_name, instructions: recipe_body });

        rest = rest_body;

        if rest.is_empty() {
            break;
        }
    }

    Ok(recipes)
}

pub fn dummy_recipes() -> Vec<Recipe> {
    let rec = vec![
            Recipe{
                name: vec![MdElement::Strong("rec1:".to_owned())],
                instructions: vec![
                    vec![MdElement::Text("a.".to_owned())],
                    vec![MdElement::Text("b.".to_owned())],
                    vec![MdElement::Text("c.".to_owned())],
                    vec![MdElement::Text("d.".to_owned())],
                    vec![MdElement::Text("e.".to_owned())],
            ]},
            Recipe{
                name: vec![MdElement::Strong("rec2:".to_owned())],
                instructions: vec![
                    vec![MdElement::Text("a.".to_owned())],
                    vec![MdElement::Text("b.".to_owned())],
                    vec![MdElement::Text("c.".to_owned())],
                    vec![MdElement::Text("d.".to_owned())],
                    vec![MdElement::Text("e.".to_owned())],
                    vec![MdElement::Text("f.".to_owned())],
            ]},
            Recipe{
                name: vec![MdElement::Strong("rec3:".to_owned())],
                instructions: vec![
                    vec![MdElement::Text("a.".to_owned())],
                    vec![MdElement::Text("b.".to_owned())],
                    vec![MdElement::Text("c.".to_owned())],
                    vec![MdElement::Text("d.".to_owned())],
            ]},
    ];
    return rec;
}



fn ordered_list_bullet(input: &str) -> IResult<&str, &str> {
    delimited(preceded(newline, multispace0), digit1, tag(". "))(input)
}

fn ordered_list_item(input: &str) -> IResult<&str, Vec<MdElement>> {
    preceded(ordered_list_bullet, md_text)(input)
}

fn unordered_list_bullet(input: &str) -> IResult<&str, &str> {
    preceded(preceded(newline, multispace0), tag("- "))(input)
}

fn unordered_list_item(input: &str) -> IResult<&str, Vec<MdElement>> {
    preceded(unordered_list_bullet, md_text)(input)
}

fn unordered_list(input: &str) -> IResult<&str, Vec<Vec<MdElement>>> {
    many1(unordered_list_item)(input)
}


fn md_emphasis(input: &str) -> IResult<&str, &str> {
    alt((
        delimited(char('*'), md_elem_inside("*"), char('*')),
        delimited(char('_'), md_elem_inside("_"), char('_')),
    ))(input)
}


fn md_strong(input: &str) -> IResult<&str, &str> {
    alt((
        // delimited(tag("**"), take_till(|c| c == '*'), tag("**")),
        delimited(tag("**"), md_elem_inside("*"), tag("**")),
        delimited(tag("__"), md_elem_inside("_"), tag("__")),
    ))(input)
}

// TODO(filip): i don't know if this is the best way to do this, just want to try
// to get this working
// TODO(filip): I really don't like the generics here. has to be a better way of doing this
fn md_elem_inside<'a, 'b, E: ParseError<&'a str> + 'b>(
    stop_tag: &'b str,
) -> impl FnMut(&'a str) -> IResult<&'a str, &'a str, E> + 'b
where
    'a: 'b
    
{
    recognize(many_till(
        anychar, 
        peek(
            alt((
                // stop_tag,
                tag(stop_tag),
                tag("\n\n")
            ))
        )))
}
//
// fn ws<'a, F, O, E: ParseError<&'a str>>(inner: F) -> impl Parser<&'a str>
fn ws<'a, F, O, E: ParseError<&'a str>>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
  where
    F: Parser<&'a str, O, E>,
{
  delimited(
    nom::character::complete::multispace0,
    inner,
    nom::character::complete::multispace0
  )
}


// TODO(filip): move to its own file
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum MdElement {
    Em(String),
    Strong(String),
    Text(String)
}

impl IntoView for MdElement {
    fn into_view(self) -> leptos::View {
        match self {
            MdElement::Em(s) => view! { <em>{s}</em> }.into_view(),
            MdElement::Strong(s) => view! { <strong>{s}</strong>}.into_view(),
            MdElement::Text(s) => s.into_view(),
        }
    }
}

type MdFragment = Vec<MdElement>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct Recipe {
    pub name: MdFragment,
    pub instructions: Vec<MdFragment>,
}

fn md_special_text(input: &str) -> IResult<&str, MdElement> {
    // bold **abc**
    // italics *abc*
    // text ahoj*jak jse mas\n\n
    if let Ok((rest, strong)) = md_strong(input) {
        return Ok((rest, MdElement::Strong(strong.to_owned())))
    };

    match md_emphasis(input) {
        Ok((rest, em)) => return Ok((rest, MdElement::Em(em.to_owned()))),
        Err(e) => return Err(e),
    };
}

fn md_plain_text(input: &str) -> IResult<&str, MdElement> {
    let (rest, t) = recognize(many_till(
        anychar,
        peek(
            alt((
                tag("\n\n"),
                tag("*"),
                tag("_"),
                // delimited(multispace0, digit1, tag(". ")),
                ordered_list_bullet,
                unordered_list_bullet,
                eof
            ))
        )
    ))(input)?;
    Ok((rest, MdElement::Text(t.to_owned())))
}

// TODO(filip): this requires to explicitly specify, what is not text anymore
// it is probably pretty expensive, since it has to keep checking if some of the parsers match for
// each byte i imagine
fn md_text(input: &str) -> IResult<&str, Vec<MdElement>> {
    let (rest, (t, _)) = many_till(
        alt((
            md_special_text,
            md_plain_text
        )),
        peek(
            alt((
                tag("\n\n"),
                eof,
                ordered_list_bullet,
                unordered_list_bullet,
            ))
        )
    )(input)?;
    println!("rest: {:?}, t: {:?}", rest, t);
    Ok((rest, t))
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ordered_list_bullet() {
        let (rest, bullet) = ordered_list_bullet("\n1. this is an item here").unwrap();
        assert_eq!(bullet, "1");
        assert_eq!(rest, "this is an item here");
    }

    #[test]
    fn test_ordered_list_item() {
        let (rest, li) = ordered_list_item("\n1. this is an item here").unwrap();
        println!("rest: {:?} li: {:?}", rest, li);
        assert_eq!(li, vec![MdElement::Text("this is an item here".to_string())]);
        assert_eq!(rest, "");
    }

    #[test]
    fn test_ordered_list_item_multiline() {
        let (rest, li) = ordered_list_item("\n1. this is an item here\nlong line").unwrap();
        assert_eq!(li, vec![MdElement::Text("this is an item here\nlong line".to_string())]);
        assert_eq!(rest, "");
    }

    #[test]
    fn test_ordered_list_item_multiline_ended() {
        let (rest, li) = ordered_list_item("\n1. this is an item here\nlong line\n\ntext").unwrap();
        assert_eq!(li, vec![MdElement::Text("this is an item here\nlong line".to_string())]);
        assert_eq!(rest, "\n\ntext");
    }

    #[test]
    fn test_ordered_list_item_empty() {
        let res = ordered_list_item("\nnot a list");
        assert!(res.is_err());
    }


    #[test]
    fn test_ordered_list_item_with_special_text() {
        let (rest, li) = ordered_list_item("\n1. this is an **item** *here\nlong *line\n\ntext").unwrap();
        assert_eq!(li, vec![
            MdElement::Text("this is an ".to_string()),
            MdElement::Strong("item".to_string()),
            MdElement::Text(" ".to_string()),
            MdElement::Em("here\nlong ".to_string()),
            MdElement::Text("line".to_string())
        ]);
        assert_eq!(rest, "\n\ntext");
    }

    #[test]
    fn test_unordered_list() {
        let (rest, li) = unordered_list("\n- this is an item here\nlong - line\n- another item\n- another\n\n").unwrap();
        assert_eq!(li, vec![
            vec![MdElement::Text("this is an item here\nlong - line".to_string())],
            vec![MdElement::Text("another item".to_string())],
            vec![MdElement::Text("another".to_string())],
        ]);
        assert_eq!(rest, "\n\n");
    }

    #[test]
    fn test_unordered_list_no_newline_end() {
        let (rest, li) = unordered_list("\n- this is an item here\nlong - line\n- another item\n- another").unwrap();
        assert_eq!(li, vec![
            vec![MdElement::Text("this is an item here\nlong - line".to_string())],
            vec![MdElement::Text("another item".to_string())],
            vec![MdElement::Text("another".to_string())],
        ]);
        assert_eq!(rest, "");
    }

    #[test]
    fn test_strong_elem() {
        let (rest, strong) = md_strong("**this is strong** and this is not").unwrap();
        assert_eq!(strong, "this is strong");
        assert_eq!(rest, " and this is not");
    }

    #[test]
    fn test_strong_elem_newline() {
        let (rest, strong) = md_strong("**this is strong\nthis should still be strong** and this is not").unwrap();
        assert_eq!(strong, "this is strong\nthis should still be strong");
        assert_eq!(rest, " and this is not");
    }

    #[test]
    fn test_md_elem_inside_delimited() {
        let res: IResult<&str, &str> = md_elem_inside("**")("this is strong\nthis should still be strong** and this is not");
        let (rest, strong) = res.unwrap();
        assert_eq!(strong, "this is strong\nthis should still be strong");
        assert_eq!(rest, "** and this is not");
    }

    #[test]
    fn test_md_elem_inside_nl_block() {
        let res: IResult<&str, &str> = md_elem_inside("**")("this is strong\nthis should still be strong\n\n and this is not");
        let (rest, strong) = res.unwrap();
        assert_eq!(strong, "this is strong\nthis should still be strong");
        assert_eq!(rest, "\n\n and this is not");
    }

    #[test]
    fn test_md_plain_text() {
        let (rest, text) = md_plain_text("this is text\nand this is also\n\nthis is not").unwrap();
        match text {
            MdElement::Em(_) => todo!(),
            MdElement::Strong(_) => todo!(),
            MdElement::Text(t) => assert_eq!(t, "this is text\nand this is also"),
        }
        assert_eq!(rest, "\n\nthis is not");
    }

    #[test]
    fn test_md_plain_text_inline() {
        let (rest, text) = md_plain_text("this is text\nand this is also").unwrap();
        match text {
            MdElement::Em(_) => todo!(),
            MdElement::Strong(_) => todo!(),
            MdElement::Text(t) => assert_eq!(t, "this is text\nand this is also"),
        }
        assert_eq!(rest, "");
    }

    #[test]
    fn test_md_text() {
        let (rest, t) = md_text("what about this *this is an item here*\n1. this not").unwrap();
        assert_eq!(t, vec![MdElement::Text("what about this ".to_string()), MdElement::Em("this is an item here".to_string())]);
        assert_eq!(rest, "\n1. this not");
    }

    #[test]
    fn test_md_text_empty() {
        let (rest, t) = md_text("\n- what about this *this is an item here*\n1. this not").unwrap();
        assert_eq!(t, vec![]);
        assert_eq!(rest, "\n- what about this *this is an item here*\n1. this not");
    }

    #[test]
    fn test_parse_recipe() {
        // let inp = "Sure! Here are a few recipe ideas using the ingredients you mentioned:\n\n1. **Ham and Potato Hash:**\n   - Dice the ham and potatoes into small cubes.\n   - Heat some oil in a pan and sauté the potatoes until they become golden and crispy.\n   - Add the diced ham and cook until it's heated through.\n   - Season with salt, pepper, and any other preferred spices.\n   - Serve hot as a delicious and simple dinner option.\n\n2. **Gluten-Free Ham Fried Rice:**\n   - Cook rice noodles according to package instructions and set aside.\n   - In a pan, heat some oil and sauté the diced ham until lightly browned.\n   - Add cooked rice noodles, canned corn, and any other desired vegetables.\n   - Stir in gluten-free soy sauce or tamari for flavor.\n   - Cook until everything is heated through and well combined.\n   - Serve hot as a tasty gluten-free alternative to fried rice.\n\n3. **Tomato and Ham Stew:**\n   - In a pot, combine tomato sauce, diced ham, and canned corn.\n   - Add any desired seasonings like garlic, onion powder, or Italian herbs.\n   - Simmer the mixture on low heat for about 15-20 minutes.\n   - Serve the stew over cooked rice noodles for a hearty and flavorful dinner.\n\nRemember to adjust the recipes according to your taste preferences and dietary restrictions. Enjoy your meal!";
        let inp = "Sure! Here are a few recipe ideas using:\n\n1. **rec1:**\n   - a.\n   - b.\n   - c.\n   - d.\n   - e.\n\n2. **rec2:**\n   - a.\n   - b.\n   - c.\n   - d.\n   - e.\n   - f.\n\n3. **rec3:**\n   - a.\n   - b.\n   - c.\n   - d.\n\nRemember to adjust. Enjoy your meal!";

        let r = parse(inp).unwrap();

        assert_eq!(r, vec![
            Recipe{
                name: vec![MdElement::Strong("rec1:".to_owned())],
                instructions: vec![
                    vec![MdElement::Text("a.".to_owned())],
                    vec![MdElement::Text("b.".to_owned())],
                    vec![MdElement::Text("c.".to_owned())],
                    vec![MdElement::Text("d.".to_owned())],
                    vec![MdElement::Text("e.".to_owned())],
            ]},
            Recipe{
                name: vec![MdElement::Strong("rec2:".to_owned())],
                instructions: vec![
                    vec![MdElement::Text("a.".to_owned())],
                    vec![MdElement::Text("b.".to_owned())],
                    vec![MdElement::Text("c.".to_owned())],
                    vec![MdElement::Text("d.".to_owned())],
                    vec![MdElement::Text("e.".to_owned())],
                    vec![MdElement::Text("f.".to_owned())],
            ]},
            Recipe{
                name: vec![MdElement::Strong("rec3:".to_owned())],
                instructions: vec![
                    vec![MdElement::Text("a.".to_owned())],
                    vec![MdElement::Text("b.".to_owned())],
                    vec![MdElement::Text("c.".to_owned())],
                    vec![MdElement::Text("d.".to_owned())],
            ]},
        ]);
    }
    
    #[test]
    fn test_parse_recipe_no_foreword() {
        let inp = "\n\n\n1. **rec1:**\n   - a.\n   - b.\n   - c.\n   - d.\n   - e.\n\n2. **rec2:**\n   - a.\n   - b.\n   - c.\n   - d.\n   - e.\n   - f.\n\n3. **rec3:**\n   - a.\n   - b.\n   - c.\n   - d.\n\nRemember to adjust. Enjoy your meal!";

        let r = parse(inp).unwrap();

        assert_eq!(r, vec![
            Recipe{
                name: vec![MdElement::Strong("rec1:".to_owned())],
                instructions: vec![
                    vec![MdElement::Text("a.".to_owned())],
                    vec![MdElement::Text("b.".to_owned())],
                    vec![MdElement::Text("c.".to_owned())],
                    vec![MdElement::Text("d.".to_owned())],
                    vec![MdElement::Text("e.".to_owned())],
            ]},
            Recipe{
                name: vec![MdElement::Strong("rec2:".to_owned())],
                instructions: vec![
                    vec![MdElement::Text("a.".to_owned())],
                    vec![MdElement::Text("b.".to_owned())],
                    vec![MdElement::Text("c.".to_owned())],
                    vec![MdElement::Text("d.".to_owned())],
                    vec![MdElement::Text("e.".to_owned())],
                    vec![MdElement::Text("f.".to_owned())],
            ]},
            Recipe{
                name: vec![MdElement::Strong("rec3:".to_owned())],
                instructions: vec![
                    vec![MdElement::Text("a.".to_owned())],
                    vec![MdElement::Text("b.".to_owned())],
                    vec![MdElement::Text("c.".to_owned())],
                    vec![MdElement::Text("d.".to_owned())],
            ]},
        ]);
    }
}

use crate::helpers::TestApp;
use html::{Dom, Element, Node};
use itertools::EitherOrBoth::{Both, Left, Right};
use itertools::Itertools;
use std::error::Error;
use tokio::fs;
use tokio::io::AsyncReadExt;

#[tokio::test]
async fn test_so_underlag() -> Result<(), Box<dyn Error>> {
    let app = TestApp::try_new().await?;

    let mut fp = fs::File::open("assets/data/facit_underlag_2.xhtml").await?;
    let mut buffer = String::new();
    fp.read_to_string(&mut buffer).await?;
    let facit_tree = Dom::parse(&buffer)?;

    let mut buf = Vec::new();
    {
        let writer = Box::new(&mut buf);
        app.so_exporter.write_so_underlag(writer, &[100001]).await?;
    }
    let output = std::str::from_utf8(buf.as_slice()).unwrap();
    println!("output: {}", output);
    let result = Dom::parse(output)?;

    assert_trees_equal(result, facit_tree);
    Ok(())
}

fn assert_trees_equal(tree: Dom, facit: Dom) {
    assert_eq!(tree.tree_type, facit.tree_type);
    for pair in tree
        .children
        .iter()
        .zip_longest(facit.children.iter().filter(|n| is_not_used(n)))
    {
        match pair {
            Both(e1, f1) => {
                assert_node_equal(e1, f1);
            }
            Left(e1) => panic!("tree under test is longer: {:?}", e1),
            Right(f1) => panic!("tree under test is missing: {:?}", f1),
        }
    }
}

fn is_not_used(n: &Node) -> bool {
    match n {
        Node::Element(e) => {
            if e.name == "script" {
                false
            } else if e.name == "link" {
                false
            } else {
                true
            }
        }
        _ => true,
    }
}

fn is_not_used_for_child(n: &Node, parent: &Element) -> bool {
    match n {
        Node::Element(e) => {
            if e.name == "script" {
                false
            } else if e.name == "link" {
                false
            } else {
                if parent.name == "body"
                    && ["span", "div_copyr", "details"].contains(&e.name.as_str())
                {
                    false
                } else {
                    true
                }
            }
        }
        _ => true,
    }
}

fn assert_node_equal(e: &Node, f: &Node) {
    match (e, f) {
        (Node::Comment(_), Node::Comment(_)) => (),
        (Node::Text(et), Node::Text(ft)) => assert_text_equal(et, ft),
        (Node::Element(ee), Node::Element(fe)) => assert_elements_equal(ee, fe),
        _ => {
            panic!("e != f, '{:?}' != '{:?}'", e, f);
        }
    }
}

fn assert_elements_equal(ee: &Element, fe: &Element) {
    tracing::debug!("{}   {}", ee.name, fe.name);
    assert_eq!(ee.name, fe.name);
    assert_eq!(ee.variant, fe.variant);

    for pair in ee
        .children
        .iter()
        .zip_longest(fe.children.iter().filter(|n| is_not_used_for_child(n, fe)))
    {
        match pair {
            Both(e1, f1) => assert_node_equal(e1, f1),
            Left(e1) => panic!("element under test is longer: {:?}", e1),
            Right(f1) => panic!("element under test is missing: {:?}", f1),
        }
    }
    // assert_eq!(ee, fe);
}

fn assert_text_equal(et: &str, ft: &str) {
    if et.contains("DOCTYPE html") && ft.contains("DOCTYPE html") {
        return;
    }
    assert_eq!(et.trim(), ft.trim());
}

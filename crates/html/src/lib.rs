mod html;

pub use html::to_string;
pub use html::Html;
pub use html_parser::{Dom, Element, Node};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

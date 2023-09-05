#[cfg(test)]
mod tests {
    use regex::Regex;
    use ssg::{
        utilities::format_header_with_id_class,
        modules::html::generate_html,
    };

    #[test]
    fn test_generate_html_with_front_matter() {
        let content = "---\ntitle: Hello, world!\ndescription: A simple greeting\n---\n# Welcome";
        let title = "Welcome";
        let description = "Say hi to the world!";
        let result = generate_html(content, title, description, None);
        let expected = "<h1 id=\"h1-welcome\" tabindex=\"0\" id=\"\" tabindex=\"0\" class=\"welcome\">Welcome</h1><p>Say hi to the world!</p><h1 id=\"h1-welcome\" tabindex=\"0\" class=\"welcome\">Welcome</h1>";
        match result {
            Ok(res) => assert_eq!(res.trim(), expected),
            Err(e) => panic!("Error: {:?}", e),
        }
    }

    #[test]
    fn test_generate_html_without_front_matter() {
        let content = "# Welcome";
        let title = "Welcome";
        let description = "Say hi to the world!";
        let result = generate_html(content, title, description, None);
        let expected = "<h1 id=\"h1-welcome\" tabindex=\"0\" id=\"\" tabindex=\"0\" class=\"welcome\">Welcome</h1><p>Say hi to the world!</p><h1 id=\"h1-welcome\" tabindex=\"0\" class=\"welcome\">Welcome</h1>";
        match result {
            Ok(res) => assert_eq!(res.trim(), expected),
            Err(e) => panic!("Error: {:?}", e),
        }
    }

    #[test]
    fn test_generate_html_without_title() {
        let content =
            "---\ndescription: A simple greeting\n---\n# Welcome";
        let title = "";
        let description = "Say hi to the world!";
        let result = generate_html(content, title, description, None);
        let expected = "<p>Say hi to the world!</p><h1 id=\"h1-welcome\" tabindex=\"0\" class=\"welcome\">Welcome</h1>";
        match result {
            Ok(res) => assert_eq!(res.trim(), expected),
            Err(e) => panic!("Error: {:?}", e),
        }
    }

    #[test]
    fn test_generate_html_without_description() {
        let content = "---\ntitle: Hello, world!\n---\n# Welcome";
        let title = "Welcome";
        let description = "";
        let result = generate_html(content, title, description, None);
        let expected = "<h1 id=\"h1-welcome\" tabindex=\"0\" id=\"\" tabindex=\"0\" class=\"welcome\">Welcome</h1><h1 id=\"h1-welcome\" tabindex=\"0\" class=\"welcome\">Welcome</h1>";
        match result {
            Ok(res) => assert_eq!(res.trim(), expected),
            Err(e) => panic!("Error: {:?}", e),
        }
    }

    #[test]
    fn test_generate_html_with_empty_fields() {
        let content = "---\ntitle:\ndescription:\n---\n# Welcome";
        let title = "Welcome";
        let description = "Say hi to the world!";
        let result = generate_html(content, title, description, None);
        let expected = "<h1 id=\"h1-welcome\" tabindex=\"0\" id=\"\" tabindex=\"0\" class=\"welcome\">Welcome</h1><p>Say hi to the world!</p><h1 id=\"h1-welcome\" tabindex=\"0\" class=\"welcome\">Welcome</h1>";
        match result {
            Ok(res) => assert_eq!(res.trim(), expected),
            Err(e) => panic!("Error: {:?}", e),
        }
    }

    #[test]
    fn test_generate_html_with_empty_content() {
        let content = "";
        let title = "Welcome";
        let description = "Say hi to the world!";
        let result = generate_html(content, title, description, None);
        let expected = "<h1 id=\"h1-welcome\" tabindex=\"0\" id=\"\" tabindex=\"0\" class=\"welcome\">Welcome</h1><p>Say hi to the world!</p>";
        match result {
            Ok(res) => assert_eq!(res.trim(), expected),
            Err(e) => panic!("Error: {:?}", e),
        }
    }

    #[test]
    fn test_format_header_with_id_class() {
        let header_str = "<h1>Hello, world!</h1>";
        let id_regex = Regex::new(r"[^a-z0-9]+").unwrap();
        let result = format_header_with_id_class(header_str, &id_regex);
        let expected = "<h1 id=\"h1-hello\" tabindex=\"0\" class=\"hello\">Hello, world!</h1>";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_format_header_with_id_class_multiple_words() {
        let header_str = "<h1>Welcome to the world</h1>";
        let id_regex = Regex::new(r"[^a-z0-9]+").unwrap();
        let result = format_header_with_id_class(header_str, &id_regex);
        let expected = "<h1 id=\"h1-welcome\" tabindex=\"0\" class=\"welcome\">Welcome to the world</h1>";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_format_header_with_id_class_special_characters() {
        let header_str = "<h1>Hello, world! #$%^&*()</h1>";
        let id_regex = Regex::new(r"[^a-z0-9]+").unwrap();
        let result = format_header_with_id_class(header_str, &id_regex);
        let expected = "<h1 id=\"h1-hello\" tabindex=\"0\" class=\"hello\">Hello, world! #$%^&*()</h1>";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_format_header_with_id_class_no_header_tag() {
        let header_str = "Hello, world!";
        let id_regex = Regex::new(r"[^a-z0-9]+").unwrap();
        let result = format_header_with_id_class(header_str, &id_regex);
        assert_eq!(result, header_str);
    }
}

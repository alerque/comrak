use comrak::{create_formatter, nodes::NodeValue};

create_formatter!(CustomFormatter, {
    NodeValue::Emph => |output, entering| {
        if entering {
            output.write_all(b"<i>")?;
        } else {
            output.write_all(b"</i>")?;
        }
    },
    NodeValue::Strong => |context, entering| {
        use std::io::Write;
        context.write_all(if entering { b"<b>" } else { b"</b>" })?;
    },
    NodeValue::Image(ref nl) => |output, node, entering, suppress_children| {
        assert!(node.data.borrow().sourcepos == (3, 1, 3, 18).into());
        if entering {
            output.write_all(nl.url.to_uppercase().as_bytes())?;
            *suppress_children = true;
        }
    },
});

fn main() {
    use comrak::{parse_document, Arena, Options};

    let options = Options::default();
    let arena = Arena::new();
    let doc = parse_document(
        &arena,
        "_Hello_, **world**.\n\n![title](/img.png)",
        &options,
    );

    let mut buf: Vec<u8> = vec![];
    CustomFormatter::format_document(doc, &options, &mut buf).unwrap();

    assert_eq!(
        std::str::from_utf8(&buf).unwrap(),
        "<p><i>Hello</i>, <b>world</b>.</p>\n<p>/IMG.PNG</p>\n"
    );
}

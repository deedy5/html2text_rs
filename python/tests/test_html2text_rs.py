import html2text_rs  # type: ignore


def test_text_markdown():
    html = "<h1>Hello World</h1><p>This is a test.</p>"
    expected_output = "# Hello World\n\nThis is a test.\n"
    result = html2text_rs.text_markdown(html, width=80)
    assert result == expected_output, (
        f"\nExpected:\n {expected_output} \nGot:\n {result}"
    )


def test_text_plain():
    html = "<h1>Hello World</h1><p>This is a test.</p>"
    expected_output = "Hello World\n\nThis is a test.\n"
    result = html2text_rs.text_plain(html, width=80)
    assert result == expected_output, (
        f"\nExpected:\n {expected_output} \nGot:\n {result}"
    )


def test_text_rich():
    html = "<h1>Hello World</h1><p>This is a test.</p>"
    expected_output = "# Hello World\n\nThis is a test.\n"
    result = html2text_rs.text_rich(html, width=80)
    assert result == expected_output, (
        f"\nExpected:\n {expected_output} \nGot:\n {result}"
    )

use html2text::{
    from_read, from_read_with_decorator,
    render::{RichDecorator, TrivialDecorator},
};
use pyo3::prelude::*;

/// Convert HTML to markdown text
#[pyfunction]
#[pyo3(signature=(html, width=100))]
fn text_markdown(html: String, width: usize, py: Python) -> PyResult<String> {
    let text = py
        .allow_threads(|| from_read(html.as_bytes(), width))
        .expect("Error when extracting markdown text");
    Ok(text)
}

/// Convert HTML to plain text
#[pyfunction]
#[pyo3(signature=(html, width=100))]
fn text_plain(html: String, width: usize, py: Python) -> PyResult<String> {
    let text = py
        .allow_threads(|| from_read_with_decorator(html.as_bytes(), width, TrivialDecorator::new()))
        .expect("Error when extracting plain text");
    Ok(text)
}

/// Convert HTML to rich text
#[pyfunction]
#[pyo3(signature=(html, width=100))]
fn text_rich(html: String, width: usize, py: Python) -> PyResult<String> {
    let text = py
        .allow_threads(|| from_read_with_decorator(html.as_bytes(), width, RichDecorator::new()))
        .expect("Error when extracting rich text");
    Ok(text)
}

#[pymodule]
fn html2text_rs(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(text_markdown, m)?)?;
    m.add_function(wrap_pyfunction!(text_plain, m)?)?;
    m.add_function(wrap_pyfunction!(text_rich, m)?)?;
    Ok(())
}

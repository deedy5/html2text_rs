use html2text::{from_read, from_read_with_decorator, render::text_renderer::TrivialDecorator};
use pyo3::prelude::*;

/// Convert HTML to markdown text
#[pyfunction]
#[pyo3(signature=(html, width=100))]
fn text_markdown(html: String, width: usize, py: Python) -> PyResult<String> {
    let text = py.allow_threads(|| from_read(html.as_bytes(), width));
    Ok(text)
}

/// Convert HTML to plain text
#[pyfunction]
#[pyo3(signature=(html, width=100))]
fn text_plain(html: String, width: usize, py: Python) -> PyResult<String> {
    let text = py.allow_threads(|| {
        from_read_with_decorator(html.as_bytes(), width, TrivialDecorator::new())
    });
    Ok(text)
}

#[pymodule]
fn html2text_rs(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(text_plain, m)?)?;
    m.add_function(wrap_pyfunction!(text_markdown, m)?)?;
    Ok(())
}

use pyo3::prelude::*;
use pyo3::exceptions::PyKeyboardInterrupt;

use config::get_public_path;
use server::Server;
use website_handler::WebsiteHandler;
use ctrlc;

mod config;
mod server;
mod website_handler;
mod http;


// Python::with_gil(|py| {
//     let result: PyResult<()> = py.run("raise KeyboardInterrupt", None, None);

//     let error_type = match result {
//         Ok(_) => "Not an error",
//         Err(error) if error.is_instance_of::<PyKeyboardInterrupt>(py) => "KeyboardInterrupt",
//         Err(_) => "Some other error",
//     };
//     assert_eq!(error_type, "KeyboardInterrupt");
// });

#[pyfunction]
fn webserver() {
    ctrlc::set_handler(|| std::process::exit(2)).unwrap();
    let public_path = get_public_path();
    Server::new("127.0.0.1:8080".to_string()).run(WebsiteHandler::new(public_path));
}

#[pymodule]
fn rustyserve(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(webserver, m)?)?;
    Ok(())
    // Err(PyKeyboardInterrupt::new_err("Keyboard Interrupt"))
}



use std::sync::Arc;

use pyo3::prelude::*;

use crazyradio2;
use serde_json::Value;

#[pyclass]
struct Crazyradio2 {
    radio: Arc<crate::crazyradio2::Crazyradio2>,
}

fn py_to_value(pobject: &PyAny) -> anyhow::Result<Value> {
    
    if pobject.is_none() {
        return Ok(Value::Null);
    }
    if let Ok(num)  = pobject.extract::<u64>() {
        return Ok(Value::Number(num.into()));
    }
    if let Ok(num)  = pobject.extract::<i64>() {
        return Ok(Value::Number(num.into()));
    }
    if let Ok(num)  = pobject.extract::<f64>() {
        return Ok(num.into());
    }
    if let Ok(num)  = pobject.extract::<bool>() {
        return Ok(Value::Bool(num));
    }
    if let Ok(num)  = pobject.extract::<String>() {
        return Ok(Value::String(num));
    }
    if let Ok(num)  = pobject.extract::<Vec<&PyAny>>() {
        let mut vec = Vec::new();
        for item in num {
            vec.push(py_to_value(&item)?);
        }
        return Ok(Value::Array(vec));
    }
    if let Ok(num)  = pobject.extract::<std::collections::HashMap<String, &PyAny>>() {
        let mut map = std::collections::HashMap::new();
        for (key, value) in num {
            map.insert(key, py_to_value(&value)?);
        }
        return Ok(Value::Object(serde_json::Map::from_iter(map.into_iter())));
    }    

    Err(anyhow::anyhow!("Unsuported type"))
}

fn value_to_py(pobject: &Value, py: Python<'_>) -> PyResult<PyObject> {
    match pobject {
        Value::Null => Ok(py.None()),
        Value::Bool(b) => Ok(b.to_object(py)),
        Value::Number(n) => Ok(n.as_f64().unwrap().to_object(py)),
        Value::String(s) => Ok(s.to_object(py)),
        Value::Array(a) => {
            let mut vec = Vec::new();
            for item in a {
                vec.push(value_to_py(&item, py)?);
            }
            Ok(vec.to_object(py))
        }
        Value::Object(o) => {
            let mut map = std::collections::HashMap::new();
            for (key, value) in o {
                map.insert(key, value_to_py(&value, py)?);
            }
            Ok(map.to_object(py))
        }
    }
}

#[pymethods]
impl Crazyradio2 {
    #[new]
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
          radio: Arc::new(crate::crazyradio2::Crazyradio2::new()?),
        })
    }

    pub fn rpc_call(&self, method: &str, params: &str) -> anyhow::Result<String> {
        let params: serde_json::Value = serde_json::from_str(params)?;
        let res: Value = self.radio.rpc.call(method, params)?;
        Ok(serde_json::to_string(&res)?)
    }

    pub fn rpc_call_py(&self, py: Python<'_>, method: &str, params: &PyAny) -> anyhow::Result<PyObject> {
        let params = py_to_value(params).unwrap();
        let res: Value = self.radio.rpc.call(method, params).unwrap();
        Ok(value_to_py(&res, py)?)
    }

    pub fn radio_mode_list(&self) -> anyhow::Result<Vec<String>> {
        Ok(self.radio.radio_mode_list()?)
    }

    pub fn set_radio_mode(&self, mode: &str) -> anyhow::Result<()> {
        self.radio.radio_mode_set(mode)?;
        Ok(())
    }

    pub fn esb_send_packet(&self, channel: u8, address: &[u8], data: &[u8]) -> anyhow::Result<()> {
        let address: &[u8; 5] = address.try_into().unwrap();
        self.radio.esb_send_packet(channel, address, data)?;
        Ok(())
    }
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
#[pyo3(name="crazyradio2")]
fn crazyradio2_module(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_class::<Crazyradio2>()?;

    Ok(())
}

use crate::{Field, LorentzVector};
use num::Complex;

#[cfg(feature = "serde_support")]
use serde::de::{Deserializer, Error, SeqAccess, Visitor};
#[cfg(feature = "serde_support")]
use serde::Deserialize;
#[cfg(feature = "serde_support")]
use std::fmt;
#[cfg(feature = "serde_support")]
use std::marker::PhantomData;

#[cfg(feature = "pyo3_support")]
use pyo3::types::{PyAny, PyFloat, PyList, PySequence, PyTuple};
#[cfg(feature = "pyo3_support")]
use pyo3::{FromPyObject, PyObject, PyResult, Python, ToPyObject};

#[cfg(feature = "cpython_support")]
use cpython::{
    exc, FromPyObject, PyDrop, PyErr, PyFloat, PyList, PyObject, PyResult, PySequence, PyTuple,
    Python, PythonObject, ToPyObject,
};

#[cfg(feature = "serde_support")]
struct LorentzVectorVisitor<T: Field> {
    _marker: PhantomData<fn() -> LorentzVector<T>>,
}

#[cfg(feature = "serde_support")]
impl<'de, T: Field + Deserialize<'de>> Visitor<'de> for LorentzVectorVisitor<T> {
    type Value = LorentzVector<T>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("four floats")
    }

    fn visit_seq<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: SeqAccess<'de>,
    {
        let t = access
            .next_element::<T>()?
            .ok_or_else(|| M::Error::custom("Cannot read t-component"))?;
        let x = access
            .next_element::<T>()?
            .ok_or_else(|| M::Error::custom("Cannot read x-component"))?;
        let y = access
            .next_element::<T>()?
            .ok_or_else(|| M::Error::custom("Cannot read y-component"))?;
        let z = access
            .next_element::<T>()?
            .ok_or_else(|| M::Error::custom("Cannot read z-component"))?;

        Ok(LorentzVector::from_args(t, x, y, z))
    }
}

#[cfg(feature = "serde")]
impl<'de, T: Field + Deserialize<'de>> Deserialize<'de> for LorentzVector<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(LorentzVectorVisitor {
            _marker: PhantomData,
        })
    }
}

#[cfg(feature = "pyo3_support")]
impl ToPyObject for LorentzVector<f64> {
    fn to_object(&self, py: Python) -> PyObject {
        PyList::new(
            py,
            &[
                PyFloat::new(py, self.t),
                PyFloat::new(py, self.x),
                PyFloat::new(py, self.y),
                PyFloat::new(py, self.z),
            ],
        )
        .to_object(py)
    }
}

#[cfg(feature = "pyo3_support")]
impl ToPyObject for LorentzVector<Complex<f64>> {
    fn to_object(&self, py: Python) -> PyObject {
        PyList::new(
            py,
            &[
                PyTuple::new(
                    py,
                    &[PyFloat::new(py, self.t.re), PyFloat::new(py, self.t.im)],
                ),
                PyTuple::new(
                    py,
                    &[PyFloat::new(py, self.x.re), PyFloat::new(py, self.x.im)],
                ),
                PyTuple::new(
                    py,
                    &[PyFloat::new(py, self.y.re), PyFloat::new(py, self.y.im)],
                ),
                PyTuple::new(
                    py,
                    &[PyFloat::new(py, self.z.re), PyFloat::new(py, self.z.im)],
                ),
            ],
        )
        .to_object(py)
    }
}

#[cfg(feature = "pyo3_support")]
impl<'s> FromPyObject<'s> for LorentzVector<Complex<f64>> {
    fn extract(obj: &'s PyAny) -> PyResult<Self> {
        let seq = obj.cast_as::<PySequence>()?;
        let mut v = Vec::new();
        for item in seq.iter()? {
            let item = item?;
            let seq = item.cast_as::<PySequence>()?;
            v.push((
                f64::extract(seq.get_item(0)?)?,
                f64::extract(seq.get_item(1)?)?,
            ));
        }

        if v.len() == 3 {
            Ok(LorentzVector::from_args(
                Complex::new(0., 0.),
                Complex::new(v[0].0, v[0].1),
                Complex::new(v[1].0, v[1].1),
                Complex::new(v[2].0, v[2].1),
            ))
        } else if v.len() == 4 {
            Ok(LorentzVector::from_args(
                Complex::new(v[0].0, v[0].1),
                Complex::new(v[1].0, v[1].1),
                Complex::new(v[2].0, v[2].1),
                Complex::new(v[3].0, v[3].1),
            ))
        } else {
            pyo3::exceptions::TypeError::into("Invalid list length for LorentzVector conversion")
        }
    }
}

#[cfg(feature = "pyo3_support")]
impl<'s> FromPyObject<'s> for LorentzVector<f64> {
    fn extract(obj: &'s PyAny) -> PyResult<Self> {
        let seq = obj.cast_as::<PySequence>()?;
        let mut v = Vec::new();
        for item in seq.iter()? {
            let item = item?;
            v.push(f64::extract(&item)?);
        }

        if v.len() == 3 {
            Ok(LorentzVector::from_args(0., v[0], v[1], v[2]))
        } else if v.len() == 4 {
            Ok(LorentzVector::from_slice(&v))
        } else {
            pyo3::exceptions::TypeError::into("Invalid list length for LorentzVector conversion")
        }
    }
}

#[cfg(feature = "cpython_support")]
impl ToPyObject for LorentzVector<f64> {
    type ObjectType = PyList;

    fn to_py_object(&self, py: Python) -> PyList {
        PyList::new(
            py,
            &[
                PyFloat::new(py, self.t).into_object(),
                PyFloat::new(py, self.x).into_object(),
                PyFloat::new(py, self.y).into_object(),
                PyFloat::new(py, self.z).into_object(),
            ],
        )
    }
}

#[cfg(feature = "cpython_support")]
impl ToPyObject for LorentzVector<Complex<f64>> {
    type ObjectType = PyList;

    fn to_py_object(&self, py: Python) -> PyList {
        PyList::new(
            py,
            &[
                PyTuple::new(
                    py,
                    &[
                        PyFloat::new(py, self.t.re).into_object(),
                        PyFloat::new(py, self.t.im).into_object(),
                    ],
                )
                .into_object(),
                PyTuple::new(
                    py,
                    &[
                        PyFloat::new(py, self.x.re).into_object(),
                        PyFloat::new(py, self.x.im).into_object(),
                    ],
                )
                .into_object(),
                PyTuple::new(
                    py,
                    &[
                        PyFloat::new(py, self.y.re).into_object(),
                        PyFloat::new(py, self.y.im).into_object(),
                    ],
                )
                .into_object(),
                PyTuple::new(
                    py,
                    &[
                        PyFloat::new(py, self.z.re).into_object(),
                        PyFloat::new(py, self.z.im).into_object(),
                    ],
                )
                .into_object(),
            ],
        )
    }
}

#[cfg(feature = "cpython_support")]
impl<'s> FromPyObject<'s> for LorentzVector<Complex<f64>> {
    fn extract(py: Python, obj: &'s PyObject) -> PyResult<Self> {
        let seq = obj.cast_as::<PySequence>(py)?;
        let mut v = Vec::new();
        for item in seq.iter(py)? {
            let item = item?;
            let seq = item.cast_as::<PySequence>(py)?;
            v.push((
                f64::extract(py, &seq.get_item(py, 0)?)?,
                f64::extract(py, &seq.get_item(py, 1)?)?,
            ));
            item.release_ref(py);
        }

        if v.len() == 3 {
            Ok(LorentzVector::from_args(
                Complex::new(0., 0.),
                Complex::new(v[0].0, v[0].1),
                Complex::new(v[1].0, v[1].1),
                Complex::new(v[2].0, v[2].1),
            ))
        } else if v.len() == 4 {
            Ok(LorentzVector::from_args(
                Complex::new(v[0].0, v[0].1),
                Complex::new(v[1].0, v[1].1),
                Complex::new(v[2].0, v[2].1),
                Complex::new(v[3].0, v[3].1),
            ))
        } else {
            Err(PyErr::new::<exc::TypeError, _>(
                py,
                "Invalid list length for LorentzVector conversion",
            ))
        }
    }
}

impl<'s> FromPyObject<'s> for LorentzVector<f64> {
    fn extract(py: Python, obj: &'s PyObject) -> PyResult<Self> {
        let seq = obj.cast_as::<PySequence>(py)?;
        let mut v = Vec::new();
        for item in seq.iter(py)? {
            let item = item?;
            v.push(f64::extract(py, &item)?);
            item.release_ref(py);
        }

        if v.len() == 3 {
            Ok(LorentzVector::from_args(0., v[0], v[1], v[2]))
        } else if v.len() == 4 {
            Ok(LorentzVector::from_slice(&v))
        } else {
            Err(PyErr::new::<exc::TypeError, _>(
                py,
                "Invalid list length for LorentzVector conversion",
            ))
        }
    }
}

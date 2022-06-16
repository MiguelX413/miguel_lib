use pyo3::exceptions::PyValueError;

use pyo3::prelude::*;
use pyo3::types::PyTuple;

fn merge_sub_intervals(sub_intervals: &mut Vec<(bool, f64, f64, bool)>) {
    sub_intervals.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    let mut index: usize = 0;
    for i in 1..sub_intervals.len() {
        if sub_intervals[index].2 >= sub_intervals[i].1 {
            sub_intervals[index].2 = sub_intervals[index].2.max(sub_intervals[i].2);
        } else {
            index += 1;
            sub_intervals[index] = sub_intervals[i];
        }
    }
    sub_intervals.truncate(index + 1);
}

/// A class used to represent intervals.
#[pyclass]
pub(crate) struct Interval {
    sub_intervals: Vec<(bool, f64, f64, bool)>,
}

#[pymethods]
impl Interval {
    #[new]
    fn py_new(sub_intervals: Option<Vec<(bool, f64, f64, bool)>>) -> PyResult<Self> {
        match sub_intervals {
            Some(mut f) => {
                if f.iter().any(|&f| f.1.is_nan() || f.2.is_nan()) {
                    Err(PyValueError::new_err("Sub-interval points cannot be NaN"))
                } else {
                    if f.iter()
                        .any(|f| (f.0 && f.1.is_infinite()) || (f.3 && f.2.is_infinite()))
                    {
                        Err(PyValueError::new_err("Interval cannot contain inf"))
                    } else {
                        if f.iter()
                            .any(|&sub_interval| sub_interval.1 > sub_interval.2)
                        {
                            Err(PyValueError::new_err(
                                "Start point of sub-interval cannot be greater than its end point",
                            ))
                        } else {
                            merge_sub_intervals(&mut f);
                            Ok(Interval { sub_intervals: f })
                        }
                    }
                }
            }
            None => Ok(Interval {
                sub_intervals: vec![],
            }),
        }
    }
    #[args(other = "*")]
    fn union(&self, other: &PyTuple) -> PyResult<Interval> {
        let mut output = self.clone();
        output.union_update(other)?;
        Ok(output)
    }
    #[args(other = "*")]
    fn union_update(&mut self, other: &PyTuple) -> PyResult<()> {
        let inputs: Vec<Interval> = other.extract()?;
        self.sub_intervals
            .extend(inputs.iter().flat_map(|f| &f.sub_intervals));
        if !inputs.is_empty() {
            merge_sub_intervals(&mut self.sub_intervals);
        }
        Ok(())
    }
    fn __contains__(&self, item: f64) -> bool {
        self.sub_intervals
            .iter()
            .any(|&f| f.1 <= item && item <= f.2)
    }
    fn __repr__(&self) -> String {
        format!(
            "Interval([{}])",
            self.sub_intervals
                .iter()
                .map(|&f| format!(
                    "({}, {}, {}, {})",
                    (|x| if x { "True" } else { "False" })(f.0),
                    f.1,
                    f.2,
                    (|x| if x { "True" } else { "False" })(f.3)
                ))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
    fn __str__(&self) -> String {
        format!(
            "({})",
            self.sub_intervals
                .iter()
                .map(|&f| format!(
                    "{}{}, {}{}",
                    (|x| if x { "[" } else { "(" })(f.0),
                    f.1,
                    f.2,
                    (|x| if x { "]" } else { ")" })(f.3)
                ))
                .collect::<Vec<String>>()
                .join(" ∪ ")
        )
    }
    fn __or__(&self, other: &Interval) -> Interval {
        let mut output = self.clone();
        output.__ior__(other);
        output
    }
    fn __ior__(&mut self, other: &Interval) {
        self.sub_intervals.append(&mut other.sub_intervals.clone());
        merge_sub_intervals(&mut self.sub_intervals);
    }
}

impl Clone for Interval {
    fn clone(&self) -> Interval {
        Interval {
            sub_intervals: self.sub_intervals.clone(),
        }
    }
}

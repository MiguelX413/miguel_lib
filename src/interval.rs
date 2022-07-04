use pyo3::basic::CompareOp;
use pyo3::exceptions::PyValueError;
use std::cmp::Ordering;

use crate::Span;
use pyo3::prelude::*;
use pyo3::types::PyTuple;

#[derive(FromPyObject)]
enum SegmentsOrSpan {
    Segments(Vec<(bool, f64, f64, bool)>),
    Span(Span),
}

fn interval_segment_cmp(a: &(bool, f64, f64, bool), b: &(bool, f64, f64, bool)) -> Ordering {
    (a.1, !a.0).partial_cmp(&(b.1, !b.0)).unwrap()
}

fn merge_segments(segments: &mut Vec<(bool, f64, f64, bool)>) {
    segments.sort_by(interval_segment_cmp);
    let mut index = 0;
    for i in 1..segments.len() {
        if (segments[index].2 > segments[i].1)
            || ((segments[index].2 == segments[i].1)
        // check for adjacence
                && ((segments[index].3) || (segments[i].0)))
        {
            // emulate max()
            if (segments[i].2 > segments[index].2)
                || ((segments[i].2 == segments[index].2) && (segments[i].3))
            {
                segments[index].2 = segments[i].2;
                segments[index].3 = segments[i].3;
            }
        } else {
            index += 1;
            segments[index] = segments[i];
        }
    }
    segments.truncate(index + 1);
}

fn validate_segment(segment: (bool, f64, f64, bool)) -> bool {
    (segment.1 < segment.2) || ((segment.1 == segment.2) && segment.0 && segment.3)
}

/// A class used to represent intervals.
#[pyclass]
pub(crate) struct Interval {
    #[pyo3(get)]
    segments: Vec<(bool, f64, f64, bool)>,
}

#[pymethods]
impl Interval {
    #[new]
    fn py_new(segments_or_span: Option<SegmentsOrSpan>) -> PyResult<Self> {
        match segments_or_span {
            Some(SegmentsOrSpan::Segments(segments)) => {
                let mut output = segments
                    .iter()
                    .filter_map(|&f| {
                        if f.1.is_nan() || f.2.is_nan() {
                            return Some(Err(PyValueError::new_err(
                                "Segment points cannot be NaN",
                            )));
                        }
                        if (f.1.is_infinite() && f.0) || (f.2.is_infinite() && f.3) {
                            return Some(Err(PyValueError::new_err("Interval cannot contain inf")));
                        }
                        if f.1 > f.2 {
                            return Some(Err(PyValueError::new_err(
                                "Start point of segment cannot be greater than its end point",
                            )));
                        }
                        if (f.1 == f.2) && (!f.0 || !f.3) {
                            return None;
                        }
                        Some(Ok(f))
                    })
                    .collect::<PyResult<Vec<(bool, f64, f64, bool)>>>()?;

                merge_segments(&mut output);
                Ok(Self { segments: output })
            }
            Some(SegmentsOrSpan::Span(span)) => Ok(Self {
                segments: span
                    .segments
                    .iter()
                    .map(|&segment| (true, segment.0 as f64, segment.1 as f64, true))
                    .collect::<Vec<(bool, f64, f64, bool)>>(),
            }),
            None => Ok(Self { segments: vec![] }),
        }
    }
    /// Return a shallow copy of an Interval.
    fn copy(&self) -> Self {
        self.clone()
    }
    #[args(others = "*")]
    fn difference(&self, others: &PyTuple) -> PyResult<Self> {
        let mut output = self.clone();
        output.difference_update(others)?;
        Ok(output)
    }
    #[args(others = "*")]
    fn difference_update(&mut self, others: &PyTuple) -> PyResult<()> {
        self.__isub__(&(Self { segments: vec![] }.union(others)?));
        Ok(())
    }
    #[args(others = "*")]
    fn intersection(&self, others: &PyTuple) -> PyResult<Self> {
        let mut output = self.clone();
        output.intersection_update(others)?;
        Ok(output)
    }
    #[args(others = "*")]
    fn intersection_update(&mut self, others: &PyTuple) -> PyResult<()> {
        others
            .extract::<Vec<Self>>()?
            .iter()
            .for_each(|input| self.__iand__(input));
        Ok(())
    }
    /// Returns True if two Intervals do not overlap.
    fn isdisjoint(&self, other: &Self) -> bool {
        let mut segments = self.segments.clone();
        segments.extend(other.segments.iter());
        segments.sort_by(interval_segment_cmp);
        let mut index = 0;
        for i in 1..segments.len() {
            if (segments[index].2 > segments[i].1)
                || ((segments[index].2 == segments[i].1)
            // check for strict overlap
                    && ((segments[index].3) && (segments[i].0)))
            {
                return false;
            } else {
                index += 1;
                segments[index] = segments[i];
            }
        }
        true
    }
    /// Return True if other contains this Interval, else False.
    fn issubset(&self, other: &Self) -> bool {
        other.segments == other.__or__(self).segments
    }
    /// Return True if this Interval contains other, else False.
    fn issuperset(&self, other: &Self) -> bool {
        other.issubset(self)
    }
    #[args(others = "*")]
    fn union(&self, others: &PyTuple) -> PyResult<Self> {
        let mut output = self.clone();
        output.union_update(others)?;
        Ok(output)
    }
    #[args(others = "*")]
    fn union_update(&mut self, others: &PyTuple) -> PyResult<()> {
        let inputs = others.extract::<Vec<Self>>()?;
        self.segments
            .extend(inputs.iter().flat_map(|f| &f.segments));
        if !inputs.is_empty() {
            merge_segments(&mut self.segments);
        }
        Ok(())
    }
    fn __and__(&self, other: &Self) -> Self {
        let mut output = Self { segments: vec![] };
        let mut next_bound = 0;
        let mut bottom_bound;
        for &x in &self.segments {
            bottom_bound = next_bound;
            for y in bottom_bound..other.segments.len() {
                if (x.2 < other.segments[y].1)
                    || ((x.2 == other.segments[y].1) && !(x.3 && other.segments[y].0))
                {
                    break;
                } else {
                    let left =
                        if (x.1 > other.segments[y].1) || ((x.1 == other.segments[y].1) && !x.0) {
                            (x.0, x.1)
                        } else {
                            (other.segments[y].0, other.segments[y].1)
                        };

                    let right =
                        if (x.2 < other.segments[y].2) || ((x.2 == other.segments[y].2) && !x.3) {
                            (x.2, x.3)
                        } else {
                            (other.segments[y].2, other.segments[y].3)
                        };

                    if validate_segment((left.0, left.1, right.0, right.1)) {
                        output.segments.push((left.0, left.1, right.0, right.1));
                    }

                    next_bound = y;
                }
            }
        }
        output
    }
    fn __iand__(&mut self, other: &Self) {
        self.segments = self.__and__(other).segments;
    }
    fn __or__(&self, other: &Self) -> Self {
        let mut output = self.clone();
        output.__ior__(other);
        output
    }
    fn __ior__(&mut self, other: &Self) {
        self.segments.extend(other.segments.iter());
        merge_segments(&mut self.segments);
    }
    fn __sub__(&self, other: &Self) -> Self {
        let mut output = Self { segments: vec![] };
        let mut next_bound = 0;
        let mut bottom_bound;
        for &x in &self.segments {
            let mut temp_left_bound = (x.0, x.1);
            bottom_bound = next_bound;
            for y in bottom_bound..other.segments.len() {
                if (x.2 < other.segments[y].1)
                    || ((x.2 == other.segments[y].1) && !(x.3 && other.segments[y].0))
                {
                    break;
                } else {
                    let temp = (
                        temp_left_bound.0,
                        temp_left_bound.1,
                        other.segments[y].1,
                        !other.segments[y].0,
                    );
                    if validate_segment(temp) {
                        output.segments.push(temp);
                    }
                    if (temp_left_bound.1 < other.segments[y].2)
                        || ((temp_left_bound.1 == other.segments[y].2) && temp_left_bound.0)
                    {
                        temp_left_bound = (!other.segments[y].3, other.segments[y].2);
                    }
                    next_bound = y + 1;
                }
            }
            let last_segment = (temp_left_bound.0, temp_left_bound.1, x.2, x.3);
            if validate_segment(last_segment) {
                output.segments.push(last_segment);
            }
        }
        output
    }
    fn __isub__(&mut self, other: &Self) {
        self.segments = self.__sub__(other).segments;
    }
    fn __contains__(&self, item: f64) -> bool {
        self.segments
            .iter()
            .any(|&f| (f.1 < item && item < f.2) || ((item == f.1 && f.0) || (item == f.2 && f.3)))
    }
    fn __repr__(&self) -> String {
        format!(
            "Interval([{}])",
            self.segments
                .iter()
                .map(|&f| format!(
                    "({}, {}, {}, {})",
                    if f.0 { "True" } else { "False" },
                    f.1,
                    f.2,
                    if f.3 { "True" } else { "False" },
                ))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
    fn __str__(&self) -> String {
        if !self.segments.is_empty() {
            self.segments
                .iter()
                .map(|&f| {
                    format!(
                        "{}{}, {}{}",
                        if f.0 { "[" } else { "(" },
                        f.1,
                        f.2,
                        if f.3 { "]" } else { ")" },
                    )
                })
                .collect::<Vec<String>>()
                .join(" ∪ ")
        } else {
            "∅".to_string()
        }
    }
    fn __richcmp__(&self, other: &Self, op: CompareOp) -> bool {
        match op {
            CompareOp::Eq => self.segments == other.segments,
            CompareOp::Ne => self.segments != other.segments,
            CompareOp::Lt => self.issubset(other) && (self.segments != other.segments),
            CompareOp::Le => self.issubset(other),
            CompareOp::Gt => self.issuperset(other) && (self.segments != other.segments),
            CompareOp::Ge => self.issuperset(other),
        }
    }
    #[classattr]
    #[allow(non_upper_case_globals)]
    const __hash__: Option<PyObject> = None;
}

impl Clone for Interval {
    fn clone(&self) -> Self {
        Self {
            segments: self.segments.clone(),
        }
    }
}

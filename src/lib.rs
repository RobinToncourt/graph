#![allow(dead_code)]

use std::rc::Rc;
use std::ops::{
    Index,
    IndexMut,
};
use std::cmp::PartialEq;

const UNKNOWN_LABEL: &str = "Unknow label.";

#[derive(Debug)]
pub enum GraphError {
    InvalidVecSizeError,
}

#[derive(Debug)]
pub struct Graph<L, T> {
    labels: Rc<Vec<L>>,
    grid: Vec<GraphRow<L, T>>,
}

impl<L, T> Graph<L, T> {
    pub fn new_empty(labels: Vec<L>) -> Self {
        let labels: Rc<Vec<L>> = Rc::new(labels);
        let mut grid: Vec<GraphRow<L, T>> = Vec::new();

        for _ in 0..labels.len() {
            grid.push(GraphRow::new_empty(Rc::clone(&labels)));
        }

        Self {
            labels,
            grid,
        }
    }
}

impl<L, T> Graph<L, T>
where
    L: PartialEq,
{
    pub fn get(&self, label: &L) -> Option<&GraphRow<L, T>> {
        let index = get_index(&self.labels, &label)?;
        self.grid.get(index)
    }

    pub fn get_connected_nodes(&self, label: L) -> Vec<&L> {
        let mut result: Vec<&L> = Vec::new();

        let row: &GraphRow<L, T> = &self[label];

        for label in &*row.labels {
            if row.get(label).unwrap().is_some() {
                result.push(&label);
            }
        }

        result
    }

    pub fn dijkstra_shortest_path(&self, _start: &L, _end: &L) -> Vec<&L> {
        todo!()
    }
}

impl<L, T> Index<L> for Graph<L, T>
where
    L: PartialEq,
{
    type Output = GraphRow<L, T>;

    fn index(&self, label: L) -> &Self::Output {
        let Some(index) = get_index(&self.labels, &label) else {
            panic!("{}", UNKNOWN_LABEL);
        };

        &self.grid[index]
    }
}

impl<L, T> IndexMut<L> for Graph<L, T>
where
    L: PartialEq,
{
    fn index_mut(&mut self, label: L) -> &mut Self::Output {
        let Some(index) = get_index(&self.labels, &label) else {
            panic!("{}", UNKNOWN_LABEL);
        };

        &mut self.grid[index]
    }
}

#[derive(Debug)]
pub struct GraphRow<L, T> {
    pub labels: Rc<Vec<L>>,
    row: Vec<Option<T>>,
}

impl<L, T> GraphRow<L, T> {
    fn new_empty(labels: Rc<Vec<L>>) -> Self {
        let len = labels.len();
        Self {
            labels,
            row: none_vec(len),
        }
    }

    pub fn set(&mut self, values: Vec<Option<T>>) -> Result<(), GraphError> {
        if self.labels.len() != values.len() {
            return Err(GraphError::InvalidVecSizeError);
        }

        self.row = values;

        Ok(())
    }

    pub fn len(&self) -> usize {
        self.labels.len()
    }
}

impl<L, T> GraphRow<L, T>
where
    L: PartialEq,
{
    pub fn get(&self, label: &L) -> Option<&Option<T>> {
        let index = get_index(&self.labels, &label)?;
        self.row.get(index)
    }
}

impl<L, T> Index<L> for GraphRow<L, T>
where
    L: PartialEq,
{
    type Output = Option<T>;

    fn index(&self, label: L) -> &Self::Output {
        let Some(index) = get_index(&self.labels, &label) else {
            panic!("{}", UNKNOWN_LABEL);
        };

        &self.row[index]
    }
}

impl<L, T> IndexMut<L> for GraphRow<L, T>
where
    L: PartialEq,
{
    fn index_mut(&mut self, label: L) -> &mut Self::Output {
        let Some(index) = get_index(&self.labels, &label) else {
            panic!("{}", UNKNOWN_LABEL);
        };

        &mut self.row[index]
    }
}

fn none_vec<T>(len: usize) -> Vec<Option<T>> {
    let mut res: Vec<Option<T>> = Vec::new();

    for _ in 0..len {
        res.push(None);
    }

    res
}

fn get_index<L>(labels: &Vec<L>, label: &L) -> Option<usize>
where
    L: PartialEq,
{
    for (i, e) in labels.iter().enumerate() {
        if label == e {
            return Some(i);
        }
    }

    None
}

#[cfg(test)]
mod test_graph {
    use super::*;


}











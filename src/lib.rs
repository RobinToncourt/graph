#![allow(dead_code)]

use std::rc::Rc;
use std::ops::{
    Index,
    IndexMut,
};
use std::cmp::PartialEq;
use std::result::Result::Ok;

use anyhow; // 1.0.94
use crate::anyhow::*;

const UNKNOWN_LABEL: &str = "Unknow label.";

#[derive(Debug)]
pub struct Graph<L, T> {
    labels: Rc<Vec<L>>,
    grid: Vec<GraphRow<L, T>>,
}

impl<L, T> Graph<L, T> {
    pub fn from_labels(labels: Vec<L>) -> Self {
        let labels: Rc<Vec<L>> = Rc::new(labels);
        let mut grid: Vec<GraphRow<L, T>> = Vec::new();

        for _ in 0..labels.len() {
            grid.push(GraphRow::from_labels(Rc::clone(&labels)));
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
    pub fn get(&self, label: &L) -> anyhow::Result<&GraphRow<L, T>> {
        let index = get_index(&self.labels, &label)?;
        Ok(self.grid.get(index).ok_or(anyhow!(UNKNOWN_LABEL))?)
    }

    pub fn get_connected_nodes(&self, label: &L) -> anyhow::Result<Vec<&L>> {
        let graph_row: &GraphRow<L, T> = self.get(label)?;
        graph_row.get_connected_nodes()
    }

    pub fn get_connected_nodes_and_path(
        &self,
        label: &L
    ) -> anyhow::Result<Vec<(&L, &T)>> {
        let graph_row: &GraphRow<L, T> = self.get(label)?;
        graph_row.get_connected_nodes_and_path()
    }

    pub fn dijkstra_shortest_path(
        &self,
        start: &L,
        _end: &L
    ) -> anyhow::Result<Vec<&L>> {
        // Liste des nodes déjà parcouru.
        // Trouver la node la plus proche qui n'est pas dans liste
        // ou qui est la node de fin.
        // Si pas la node de fin, recommencer avec cette node.
        
        let visited_nodes: Vec<&L> = vec![start];
        
        let actual_node: &L = start;
        let connected_nodes: Vec<&L> = self.get_connected_nodes(actual_node)?;
        let _filtered_connected_nodes: Vec<&L> = connected_nodes.into_iter()
            .filter(|node| !visited_nodes.contains(node))
            .collect();
        
        
        
        todo!()
    }
}

impl<L, T> Index<L> for Graph<L, T>
where
    L: PartialEq,
{
    type Output = GraphRow<L, T>;

    fn index(&self, label: L) -> &Self::Output {
        match get_index(&self.labels, &label) {
            Ok(index) => &self.grid[index],
            Err(e) => panic!("{e}"),
        }
    }
}

impl<L, T> IndexMut<L> for Graph<L, T>
where
    L: PartialEq,
{
    fn index_mut(&mut self, label: L) -> &mut Self::Output {
        match get_index(&self.labels, &label) {
            Ok(index) => &mut self.grid[index],
            Err(e) => panic!("{e}"),
        }
    }
}

#[derive(Debug)]
pub struct GraphRow<L, T> {
    pub labels: Rc<Vec<L>>,
    row: Vec<Option<T>>,
}

impl<L, T> GraphRow<L, T> {
    fn from_labels(labels: Rc<Vec<L>>) -> Self {
        let len = labels.len();
        Self {
            labels,
            row: none_vec(len),
        }
    }

    pub fn len(&self) -> usize {
        self.labels.len()
    }
}

impl<L, T> GraphRow<L, T> {
    pub fn get_connected_nodes(&self) -> anyhow::Result<Vec<&L>> {
        let mut result: Vec<&L> = Vec::new();
        
        for (index, label) in self.labels.iter().enumerate() {
            if self.row[index].is_some() {
                result.push(label);
            }
        }
        
        Ok(result)
    }
    
    pub fn get_connected_nodes_and_path(&self) -> anyhow::Result<Vec<(&L, &T)>> {
        let mut result: Vec<(&L, &T)> = Vec::new();
        
        for (index, label) in self.labels.iter().enumerate() {
            if self.row[index].is_some() {
                result.push((label, self.row[index].as_ref().unwrap()));
            }
        }
        
        Ok(result)
    }
}

impl<L, T> GraphRow<L, T>
where
    L: PartialEq,
{
    pub fn get(&self, label: &L) -> anyhow::Result<&Option<T>> {
        let index = get_index(&self.labels, &label)?;
        Ok(self.row.get(index).ok_or(anyhow!(UNKNOWN_LABEL))?)
    }
    
    pub fn set(&mut self, label: &L, value: T) -> anyhow::Result<()> {
        match get_index(&self.labels, &label) {
            Ok(index) => {
                self.row[index] = Some(value);
                Ok(())
            }
            Err(e) => panic!("{e}"),
        }
    }
}

impl<L, T> Index<L> for GraphRow<L, T>
where
    L: PartialEq,
{
    type Output = Option<T>;

    fn index(&self, label: L) -> &Self::Output {
        match get_index(&self.labels, &label) {
            Ok(index) => &self.row[index],
            Err(e) => panic!("{e}"),
        }
    }
}

impl<L, T> IndexMut<L> for GraphRow<L, T>
where
    L: PartialEq,
{
    fn index_mut(&mut self, label: L) -> &mut Self::Output {
        match get_index(&self.labels, &label) {
            Ok(index) => &mut self.row[index],
            Err(e) => panic!("{e}"),
        }
    }
}

fn none_vec<T>(len: usize) -> Vec<Option<T>> {
    let mut res: Vec<Option<T>> = Vec::new();

    for _ in 0..len {
        res.push(None);
    }

    res
}

fn get_index<L>(labels: &Vec<L>, label: &L) -> anyhow::Result<usize>
where
    L: PartialEq,
{
    for (i, e) in labels.iter().enumerate() {
        if label == e {
            return Ok(i);
        }
    }

    Err(anyhow!(UNKNOWN_LABEL))
}

fn main() {
    
}

#[cfg(test)]
mod test_graph {
    use super::*;

    macro_rules! s {
        ($str: expr) => {
            $str.to_string()
        }
    }

    fn create_graph_wiki() -> Graph<String, usize> {
        let labels: Vec<String> = vec!["1", "2", "3", "4", "5", "6"].into_iter()
            .map(str::to_string)
            .collect();
    
        let mut graph: Graph<String, usize> = Graph::from_labels(labels);
        graph[s!("1")][s!("2")] = Some(7);
        graph[s!("1")][s!("3")] = Some(9);
        graph[s!("1")][s!("6")] = Some(14);
        
        graph[s!("2")][s!("1")] = Some(7);
        graph[s!("2")][s!("3")] = Some(10);
        graph[s!("2")][s!("4")] = Some(15);
        
        graph[s!("3")][s!("1")] = Some(9);
        graph[s!("3")][s!("2")] = Some(10);
        graph[s!("3")][s!("4")] = Some(11);
        graph[s!("3")][s!("6")] = Some(2);
        
        graph[s!("4")][s!("2")] = Some(15);
        graph[s!("4")][s!("3")] = Some(11);
        graph[s!("4")][s!("5")] = Some(6);
        
        graph[s!("5")][s!("4")] = Some(6);
        graph[s!("5")][s!("6")] = Some(9);
        
        graph[s!("6")][s!("1")] = Some(14);
        graph[s!("6")][s!("3")] = Some(2);
        graph[s!("6")][s!("5")] = Some(9);
        
        graph
    }
    
    fn create_graph_wiki_2() -> Graph<String, usize> {
        let labels: Vec<String> = vec!["A", "B", "C", "D", "E", "F"].into_iter()
            .map(str::to_string)
            .collect();
    
        let mut graph: Graph<String, usize> = Graph::from_labels(labels);
        graph[s!("A")][s!("B")] = Some(4);
        graph[s!("A")][s!("C")] = Some(2);
        
        graph[s!("B")][s!("C")] = Some(5);
        graph[s!("B")][s!("D")] = Some(10);
        
        graph[s!("C")][s!("E")] = Some(3);
        
        graph[s!("D")][s!("F")] = Some(11);
        
        graph[s!("E")][s!("D")] = Some(4);
        
        graph
    }
    
    #[test]
    fn test_get_index() {
        let labels = vec!["1", "2", "3", "4", "5", "6"];
        
        assert_eq!(get_index(&labels, &"1").unwrap(), 0);
        assert_eq!(get_index(&labels, &"6").unwrap(), 5);
        let err: &str = get_index(&labels, &"7").unwrap_err().downcast::<&str>().unwrap();
        assert_eq!(err, UNKNOWN_LABEL);
    }
    
    #[test]
    #[should_panic]
    fn test_get() {
        let graph: Graph<String, usize> = create_graph_wiki();
        
        assert_eq!(graph.get(&s!("3")).unwrap().get(&s!("6")).unwrap(), &Some(2));
        assert_eq!(graph.get(&s!("2")).unwrap().get(&s!("4")).unwrap(), &Some(15));
        graph.get(&s!("abc")).unwrap().get(&s!("def")).unwrap(); // panic -> pas de node à ce nom.
        graph.get(&s!("1")).unwrap().get(&s!("def")).unwrap(); // panic -> pas de node à ce nom.
        
        assert_eq!(graph[s!("1")][s!("2")], Some(7));
        assert_eq!(graph[s!("5")][s!("4")], Some(6));
        graph[s!("7")][s!("8")]; // panic -> pas de node à ce nom.
    }
    
    #[test]
    #[should_panic]
    fn test_set() {
        let mut graph: Graph<String, usize> = create_graph_wiki();
        
        graph[s!("1")][s!("2")] = Some(12);
        assert_eq!(graph[s!("1")][s!("2")], Some(12));
        graph[s!("2")][s!("3")] = Some(42);
        assert_eq!(graph[s!("2")][s!("3")], Some(42));
        graph[s!("7")][s!("8")] = None; // panic -> pas de node à ce nom.
    }
    
    #[test]
    fn test_get_connected_nodes() {
        let graph: Graph<String, usize> = create_graph_wiki();
        let expected: Vec<String> = vec!["2", "3", "6"].into_iter()
            .map(str::to_string)
            .collect();
        let expected = expected.iter().collect::<Vec<&String>>();
        
        assert_eq!(graph.get_connected_nodes(&s!("1")).unwrap(), expected);
    }
    
    #[test]
    fn test_get_connected_nodes_and_path() {
        let graph: Graph<String, usize> = create_graph_wiki();
        let deux = "2".to_string();
        let trois = "3".to_string();
        let six = "6".to_string();
        let expected: Vec<(&String, &usize)> = vec![
            (&deux, &7),
            (&trois, &9),
            (&six, &14),
        ];
        
        assert_eq!(graph.get_connected_nodes_and_path(&s!("1")).unwrap(), expected);
    }
}

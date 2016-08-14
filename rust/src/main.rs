// from https://gist.github.com/Manishearth/5fc73c405641162f0712951c387ee67c
// Discussed at https://news.ycombinator.com/item?id=11878759
// should be src/main.rs

extern crate time;

use time::{precise_time_ns};
use std::{cmp};

use tree::{Node};

mod tree {
    use std::io::{BufReader, BufRead};
    use std::fs::File;
    use std::path::Path;
    use std::{mem};
    use std::cell::{Cell};
    type Route = (usize, i32);
    pub struct Node {
        routes: Box<[Route]>,
    }

    impl Node {
        pub fn routes(&self) -> &[(usize, i32)] {
            unsafe { mem::transmute(&self.routes[..]) }
        }
    }

    pub struct Tree {
        nodes: Box<[Node]>,
    }

    impl Tree {
        pub fn nodes(&self) -> &[Node] {
            &self.nodes[..]
        }
    }

    pub fn read_places() -> Tree {
        let path = Path::new("agraph");
        let file = BufReader::new(File::open(&path).unwrap());
        let mut lines = file.lines().map(|x| x.expect("IO error"));
        let numnodes = match lines.next() {
            Some(num) => num.trim().parse().unwrap(),
            _ => panic!("Error, first line of file should describe the amount of nodes")
        };

        let mut nodes = Vec::with_capacity(numnodes);

        let mut vec_nodes = Vec::with_capacity(numnodes);
        for _ in 0..numnodes {
            nodes.push(Node {
                routes: vec!().into_boxed_slice(),
            });
            vec_nodes.push(vec!())
        }

        let mut nodes = nodes.into_boxed_slice();
        let mut vec_nodes: Vec<Vec<Route>> = vec_nodes;

        for line in lines {
            let nums: Vec<&str> = line.trim().split(' ').collect();
            let src: usize  = nums[0].parse().expect("Error: node id was not a uint");
            let dest: usize = nums[1].parse().expect("Error: neighbour id was not an int");
            let cost: i32 = nums[2].parse().expect("Error: route cost was not an int");
            vec_nodes[src].push((dest, cost));
        }

        for (vec_node, node) in vec_nodes.into_iter().zip(nodes.iter_mut()) {
            node.routes = vec_node.into_boxed_slice();
        }

        Tree { nodes: nodes }
    }
}

fn get_longest_path(nodes: &[Node], c_node: usize, mut visited: u32) -> i32 {
    let mut max = 0;

    visited = visited | (1 << (c_node as u32));

    for &(neighbor, cost) in nodes[c_node].routes().iter() {
    //for &(neighbor, cost) in unsafe{nodes.get_unchecked(c_node)}.routes().iter() {
        if (visited & (1 << (neighbor as u32))) == 0 {
            let dist = cost + get_longest_path(nodes, neighbor, visited);
            max = cmp::max(max, dist);
        }
    }

    max
}

fn main() {
    let tree = tree::read_places();
    let start_time = precise_time_ns();
    let path = get_longest_path(tree.nodes(), 0, 0);
    let duration = (precise_time_ns() - start_time) / 1000000;
    println!("{} LANGUAGE Rust {}", path, duration);
}

// from https://gist.github.com/Manishearth/5fc73c405641162f0712951c387ee67c
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
    type Route = (*const Node, i32);
    pub struct Node {
        routes: Box<[Route]>,
        pub visited: Cell<bool>,
    }

    impl Node {
        pub fn routes(&self) -> &[(&Node, i32)] {
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
                visited: Cell::new(false),
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
            vec_nodes[src].push((&nodes[dest] as *const Node, cost));
        }

        for (vec_node, node) in vec_nodes.into_iter().zip(nodes.iter_mut()) {
            node.routes = vec_node.into_boxed_slice();
        }

        Tree { nodes: nodes }
    }
}

fn get_longest_path(nodes: &[Node], cur: &Node) -> i32 {
    let mut max = 0;

    cur.visited.set(true);

    for &(neighbor, cost) in cur.routes().iter() {
        if !neighbor.visited.get() {
            let dist = cost + get_longest_path(nodes, neighbor);
            max = cmp::max(max, dist);
        }
    }

    cur.visited.set(false);

    max
}

fn main() {
    let tree = tree::read_places();
    let start_time = precise_time_ns();
    let path = get_longest_path(tree.nodes(), &tree.nodes()[0]);
    let duration = (precise_time_ns() - start_time) / 1000000;
    println!("{} LANGUAGE Rust {}", path, duration);
}

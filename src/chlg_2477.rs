/***************************************************************************************************
    * 2477. Minimum Fuel Cost
    *
    * https://leetcode.com/contest/weekly-contest-247/problems/minimum-fuel-cost/
    *
    * You are given an undirected weighted connected graph. You are given an integer n which is the
    * number of nodes in the graph and an array edges, where each edges[i] = [ui, vi, weighti] denotes
    * that there is an edge between ui and vi with weight equal to weighti.
    *
    * A path from node start to node end is a sequence of nodes [z0, z1, z2, ..., zk] such that
    * z0 = start and zk = end and there is an edge between zi and zi+1 where 0 <= i <= k-1.
    *
    * The distance of a path is the sum of the weights on the edges of the path. Let distanceToLastNode(x)
    * denote the shortest distance of a path between node x and the last node in the graph.
    *
    * A valid path is a path that starts at node 0 and ends at node n - 1.
    *
    * The fuel cost of a valid path is the sum of the distances of each node in the path. For example,
    * the fuel cost of the path [0,1,2] is distanceToLastNode(0) + distanceToLastNode(1) + distanceToLastNode(2).
    *
    * Return the minimum fuel cost to go from node 0 to node n - 1. If it is not possible to go from node 0 to node n - 1, return -1.
    *
    * Note: There will be at most 50000 edges.
    *
    * Example 1:
    * Input: n = 3, edges = [[0,1,2],[1,2,4],[2,0,8],[1,0,16]]
    * Output: 6
    * Explanation: The path 0 -> 1 -> 2 has a fuel cost of 2 + 4 + 0 = 6.
    *
    * Example 2:
    * Input: n = 4, edges = [[0,1,1],[0,2,5],[1,2,1],[2,3,1]]
    * Output: 6
    * Explanation: The path 0 -> 1 -> 2 -> 3 has a fuel cost of 1 + 1 + 1 + 1 = 4.
    * There is no path that produces a lower fuel cost.
    *
    * Constraints:
    * 2 <= n <= 105
    * 0 <= edges.length <= min(105, n * (n - 1) / 2)
    * edges[i].length == 3
    * 0 <= ui, vi < n
    * ui != vi
    * 1 <= weighti <= 105
    * There is at most one edge between any two nodes.
    * The graph is connected.
    *
    **************************************************************************************************/
use std::collections::VecDeque;
pub fn minimum_fuel_cost(roads: Vec<Vec<i32>>, seats: i32) -> i64 {
    let n = roads.len() + 1;
    let mut adj = vec![Vec::new(); n];
    let mut in_degree = vec![0; n];

    for road in roads {
        let (a, b) = (road[0] as usize, road[1] as usize);
        adj[a].push(b);
        adj[b].push(a);
        in_degree[a] += 1;
        in_degree[b] += 1;
    }

    let mut q = VecDeque::new();
    for i in 0..n {
        if in_degree[i] == 1 {
            q.push_back(i);
        }
    }

    let mut total_fuel = 0;
    while !q.is_empty() {
        let size = q.len();
        for _ in 0..size {
            let node = q.pop_front().unwrap();
            total_fuel += ((n as i32 - in_degree[node]) as i64 + seats as i64 - 1) / seats as i64;
            in_degree[node] = 0;
            for &neighbour in &adj[node] {
                in_degree[neighbour] -= 1;
                if in_degree[neighbour] == 1 {
                    q.push_back(neighbour);
                }
            }
        }
    }
    total_fuel
}


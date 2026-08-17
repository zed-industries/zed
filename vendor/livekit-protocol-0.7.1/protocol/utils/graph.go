// Copyright 2023 LiveKit, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

package utils

import (
	"container/heap"
	"log"
	"math"

	"github.com/gammazero/deque"
)

type GraphNodeProps[K comparable] interface {
	ID() K
}

type GraphEdgeProps interface {
	Length() int64
}

type SimpleGraphEdge struct{}

func (e SimpleGraphEdge) Length() int64 { return 1 }

type Graph[K comparable, N GraphNodeProps[K], E GraphEdgeProps] struct {
	nodesByID   map[K]*GraphNode[N]
	freeIndices *deque.Deque[int]
	nodes       []*GraphNode[N]
	edges       [][]*GraphEdge[N, E]
}

func NewGraph[K comparable, N GraphNodeProps[K], E GraphEdgeProps]() *Graph[K, N, E] {
	return &Graph[K, N, E]{
		nodesByID:   map[K]*GraphNode[N]{},
		freeIndices: &deque.Deque[int]{},
	}
}

func (g *Graph[K, N, E]) Size() int {
	return len(g.nodes)
}

func (g *Graph[K, N, E]) NodeIDs() []K {
	ids := make([]K, 0, len(g.nodes)-g.freeIndices.Len())
	for _, n := range g.nodes {
		if n != nil {
			ids = append(ids, n.props.ID())
		}
	}
	return ids
}

func (g *Graph[K, N, E]) InsertNode(props N) {
	if n, ok := g.nodesByID[props.ID()]; ok {
		n.props = props
		return
	}

	var i int
	if g.freeIndices.Len() != 0 {
		i = g.freeIndices.PopBack()
	} else {
		i = len(g.nodes)
		g.nodes = append(g.nodes, nil)
		for j := range g.edges {
			g.edges[j] = append(g.edges[j], nil)
		}
		g.edges = append(g.edges, make([]*GraphEdge[N, E], len(g.nodes)))
	}

	n := &GraphNode[N]{
		i:     i,
		props: props,
	}

	g.nodes[i] = n
	g.nodesByID[props.ID()] = n
}

func (g *Graph[K, N, E]) DeleteNode(id K) {
	n, ok := g.nodesByID[id]
	if !ok {
		return
	}

	delete(g.nodesByID, id)
	g.nodes[n.i] = nil

	for _, es := range g.edges {
		es[n.i] = nil
	}
	for j := range g.edges[n.i] {
		g.edges[n.i][j] = nil
	}

	g.freeIndices.PushBack(n.i)
}

func (g *Graph[K, N, E]) InsertEdge(src, dst K, props E) {
	s := g.nodesByID[src]
	d := g.nodesByID[dst]

	g.edges[s.i][d.i] = &GraphEdge[N, E]{props}
}

func (g *Graph[K, N, E]) DeleteEdge(src, dst K) {
	s := g.nodesByID[src]
	d := g.nodesByID[dst]

	g.edges[s.i][d.i] = nil
}

func (g *Graph[K, N, E]) HasNode(id K) bool {
	return g.nodesByID[id] != nil
}

func (g *Graph[K, N, E]) Node(id K) (props N) {
	n := g.nodesByID[id]
	if n == nil {
		return
	}
	return n.props
}

func (g *Graph[K, N, E]) HasEdge(src, dst K) bool {
	s := g.nodesByID[src]
	d := g.nodesByID[dst]
	if s == nil || d == nil {
		return false
	}

	return g.edges[s.i][d.i] != nil
}

func (g *Graph[K, N, E]) Edge(src, dst K) (p E) {
	s := g.nodesByID[src]
	d := g.nodesByID[dst]
	if s == nil || d == nil {
		return
	}

	e := g.edges[s.i][d.i]
	if e == nil {
		return
	}
	return e.props
}

func (g *Graph[K, N, E]) OutEdges(src K) map[K]E {
	s := g.nodesByID[src]
	if s == nil {
		return nil
	}

	edges := make(map[K]E, len(g.nodes))
	for i, e := range g.edges[s.i] {
		if e != nil {
			edges[g.nodes[i].props.ID()] = e.props
		}
	}
	return edges
}

func (g *Graph[K, N, E]) InEdges(dst K) map[K]E {
	d := g.nodesByID[dst]
	if d == nil {
		return nil
	}

	edges := make(map[K]E, len(g.nodes))
	for i, es := range g.edges {
		if es[d.i] != nil {
			edges[g.nodes[i].props.ID()] = es[d.i].props
		}
	}
	return edges
}

func (g *Graph[K, N, E]) ShortestPath(src, dst K) ([]N, int64) {
	paths := &graphPathMinHeap[N]{}
	visited := map[*GraphNode[N]]*graphPath[N]{}

	s := g.nodesByID[src]
	d := g.nodesByID[dst]
	if s == nil || d == nil {
		return nil, 0
	}

	path := &graphPath[N]{node: s}
	heap.Push(paths, path)
	visited[path.node] = path

	for {
		if paths.Len() == 0 {
			return nil, 0
		}

		prev := heap.Pop(paths).(*graphPath[N])
		for i, e := range g.edges[prev.node.i] {
			if e == nil {
				continue
			}

			path := &graphPath[N]{
				prev:   prev,
				node:   g.nodes[i],
				length: prev.length + e.props.Length(),
				num:    prev.num + 1,
			}

			if p, ok := visited[path.node]; ok && p.Less(path) {
				continue
			}
			visited[path.node] = path

			if path.node == d {
				return path.Nodes(), path.length
			}

			heap.Push(paths, path)
		}
	}
}

func (g *Graph[K, N, E]) TopologicalSort() []N {
	if g.Size() == 0 {
		return nil
	}

	log.Println(len(g.nodes))
	nodes := make([]N, 0, len(g.nodes))
	acyclic := true

	temporary := make(map[*GraphNode[N]]struct{}, len(g.nodes))
	permanent := make(map[*GraphNode[N]]struct{}, len(g.nodes))

	for _, n := range g.nodes {
		if _, ok := permanent[n]; ok {
			continue
		}

		g.traverseDepthFirst(n, func(n *GraphNode[N], next func()) {
			if _, ok := permanent[n]; ok {
				return
			}
			if _, ok := temporary[n]; ok {
				acyclic = false
				return
			}

			temporary[n] = struct{}{}

			next()

			delete(temporary, n)
			permanent[n] = struct{}{}
			nodes = append(nodes, n.props)
		})
	}

	if !acyclic {
		return nil
	}

	for i := 0; i < len(nodes)/2; i++ {
		nodes[i], nodes[len(nodes)-1-i] = nodes[len(nodes)-1-i], nodes[i]
	}
	return nodes
}

func (g *Graph[K, N, E]) traverseDepthFirst(n *GraphNode[N], fn func(n *GraphNode[N], next func())) {
	fn(n, func() {
		for i, e := range g.edges[n.i] {
			if e != nil {
				g.traverseDepthFirst(g.nodes[i], fn)
			}
		}
	})
}

type graphPath[T any] struct {
	prev   *graphPath[T]
	node   *GraphNode[T]
	length int64
	num    int
}

func (p *graphPath[T]) nodes(i int) []T {
	if p.prev == nil {
		return append(make([]T, 0, i), p.node.props)
	} else {
		return append(p.prev.nodes(i+1), p.node.props)
	}
}

func (p *graphPath[T]) Nodes() []T {
	return p.nodes(1)
}

func (p *graphPath[T]) Less(o *graphPath[T]) bool {
	return (p.length == o.length && p.num < o.num) || p.length < o.length
}

type graphPathMinHeap[T any] []*graphPath[T]

func (h *graphPathMinHeap[T]) Len() int {
	return len(*h)
}

func (h *graphPathMinHeap[T]) Less(i, j int) bool {
	return (*h)[i].Less((*h)[j])
}

func (h *graphPathMinHeap[T]) Swap(i, j int) {
	(*h)[i], (*h)[j] = (*h)[j], (*h)[i]
}

func (h *graphPathMinHeap[T]) Push(x any) {
	*h = append(*h, x.(*graphPath[T]))
}

func (h *graphPathMinHeap[T]) Pop() any {
	x := (*h)[len(*h)-1]
	(*h)[len(*h)-1] = nil
	*h = (*h)[:len(*h)-1]
	return x
}

type GraphNode[T any] struct {
	i     int
	props T
}

type GraphEdge[N, E any] struct {
	props E
}

const inf = int64(math.MaxInt64/2 - 1)

func NewFlowGraph(n int64) FlowGraph {
	cap := make([]int64, n*n)
	cost := make([]int64, n*n)
	return FlowGraph{n, cap, cost}
}

type FlowGraph struct {
	n         int64
	cap, cost []int64
}

func (g *FlowGraph) AddEdge(s, t, cap, cost int64) {
	g.cap[s*g.n+t] = cap
	g.cap[t*g.n+s] = cap
	g.cost[s*g.n+t] = cost
	g.cost[t*g.n+s] = cost
}

type MinCostMaxFlow struct {
	found           []bool
	n               int64
	cap, flow, cost []int64
	prev, dist, pi  []int64
}

func (f *MinCostMaxFlow) search(s, t int64) bool {
	for i := range f.found {
		f.found[i] = false
	}
	for i := range f.dist {
		f.dist[i] = inf
	}

	f.dist[s] = 0

	for s != f.n {
		best := f.n
		f.found[s] = true

		for i := int64(0); i < f.n; i++ {
			if f.found[i] {
				continue
			}

			if f.flow[i*f.n+s] != 0 {
				val := f.dist[s] + f.pi[s] - f.pi[i] - f.cost[i*f.n+s]
				if f.dist[i] > val {
					f.dist[i] = val
					f.prev[i] = s
				}
			}

			if f.flow[s*f.n+i] < f.cap[s*f.n+i] {
				val := f.dist[s] + f.pi[s] - f.pi[i] + f.cost[s*f.n+i]
				if f.dist[i] > val {
					f.dist[i] = val
					f.prev[i] = s
				}
			}

			if f.dist[i] < f.dist[best] {
				best = i
			}
		}

		s = best
	}

	for i := int64(0); i < f.n; i++ {
		pi := f.pi[i] + f.dist[i]
		if pi > inf {
			pi = inf
		}
		f.pi[i] = pi
	}

	return f.found[t]
}

func (f *MinCostMaxFlow) Flow(s, t int64) int64 {
	return f.flow[s*f.n+t]
}

func (f *MinCostMaxFlow) ComputeMaxFlow(g FlowGraph, s, t int64) (flow, cost int64) {
	f.cap = g.cap
	f.cost = g.cost
	f.n = g.n

	f.found = make([]bool, f.n)
	f.flow = make([]int64, f.n*f.n)
	f.dist = make([]int64, f.n+1)
	f.prev = make([]int64, f.n)
	f.pi = make([]int64, f.n)

	for f.search(s, t) {
		pathFlow := inf
		for u := t; u != s; u = f.prev[u] {
			var pf int64
			if f.flow[u*f.n+f.prev[u]] != 0 {
				pf = f.flow[u*f.n+f.prev[u]]
			} else {
				pf = f.cap[f.prev[u]*f.n+u] - f.flow[f.prev[u]*f.n+u]
			}
			if pf < pathFlow {
				pathFlow = pf
			}
		}

		for u := t; u != s; u = f.prev[u] {
			if f.flow[u*f.n+f.prev[u]] != 0 {
				f.flow[u*f.n+f.prev[u]] -= pathFlow
				cost -= pathFlow * f.cost[u*f.n+f.prev[u]]
			} else {
				f.flow[f.prev[u]*f.n+u] += pathFlow
				cost += pathFlow * f.cost[f.prev[u]*f.n+u]
			}
		}
		flow += pathFlow
	}

	return flow, cost
}

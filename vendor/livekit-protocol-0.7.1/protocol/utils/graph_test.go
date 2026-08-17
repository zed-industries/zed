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
	"testing"

	"github.com/stretchr/testify/require"
	"golang.org/x/exp/maps"
)

type testNode struct {
	id string
}

func (n *testNode) ID() string {
	return n.id
}

type testEdge int

func (e testEdge) Length() int64 {
	return int64(e)
}

func testNodeIDs[K comparable, N GraphNodeProps[K]](path []N) []K {
	var names []K
	for _, p := range path {
		names = append(names, p.ID())
	}
	return names
}

func TestGraph(t *testing.T) {
	t.Run("graph mutation", func(t *testing.T) {
		g := NewGraph[string, *testNode, testEdge]()
		g.InsertNode(&testNode{"a"})
		g.InsertNode(&testNode{"b"})
		g.InsertNode(&testNode{"c"})

		require.EqualValues(t, &testNode{"a"}, g.Node("a"))

		g.InsertEdge("a", "b", 1)
		g.InsertEdge("b", "c", 1)
		g.InsertEdge("c", "a", 1)

		require.Equal(t, []string{"b"}, maps.Keys(g.OutEdges("a")))
		require.Equal(t, []string{"c"}, maps.Keys(g.InEdges("a")))

		require.EqualValues(t, 1, g.Edge("a", "b"))

		g.DeleteEdge("a", "b")
		require.False(t, g.HasEdge("a", "b"))
	})

	t.Run("topological sort", func(t *testing.T) {
		g := NewGraph[string, *testNode, testEdge]()
		g.InsertNode(&testNode{"a"})
		g.InsertNode(&testNode{"b"})
		g.InsertNode(&testNode{"c"})
		g.InsertNode(&testNode{"d"})
		g.InsertNode(&testNode{"e"})

		g.InsertEdge("a", "b", 1)
		g.InsertEdge("b", "c", 1)
		g.InsertEdge("c", "d", 1)
		g.InsertEdge("d", "e", 1)

		require.Equal(t, []string{"a", "b", "c", "d", "e"}, testNodeIDs[string](g.TopologicalSort()))
	})

	t.Run("shortest path", func(t *testing.T) {
		g := NewGraph[string, *testNode, testEdge]()
		g.InsertNode(&testNode{"a"})
		g.InsertNode(&testNode{"b"})
		g.InsertNode(&testNode{"c"})
		g.InsertNode(&testNode{"d"})

		g.InsertEdge("a", "b", 1)
		g.InsertEdge("a", "c", 3)
		g.InsertEdge("b", "d", 2)
		g.InsertEdge("c", "d", 1)

		p, n := g.ShortestPath("a", "d")
		require.Equal(t, []string{"a", "b", "d"}, testNodeIDs[string](p))
		require.EqualValues(t, 3, n)
	})
}

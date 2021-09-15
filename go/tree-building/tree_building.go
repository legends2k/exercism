// Package tree constructs a tree from an array of nodes
package tree

import (
	"errors"
	"fmt"
	"sort"
)

type Record struct {
	ID, Parent int
}

type Node struct {
	ID       int
	Children []*Node
}

func validate(r Record, tree []Node) error {
	if r.ID >= len(tree) {
		return errors.New("invalid node ID")
	} else if (r.ID != 0) && (r.ID <= r.Parent) {
		// if not root, confirm if parent ID < ID
		return errors.New("invalid parent ID")
	} else if (r.ID == 0) && (r.Parent != 0) {
		// make sure root has itself as parent
		return errors.New("invalid root parent")
	} else if tree[r.ID].ID != -1 {
		// make sure it’s really absent; placeholder doesn’t count
		return errors.New("duplicate node")
	}
	return nil
}

func appendChild(parent *Node, child *Node) {
	parent.Children = append(parent.Children, child)
	// child IDs should be sorted; sort from behind till it meets the right spot
	i := len(parent.Children) - 1
	for ; (i >= 1) && (parent.Children[i].ID < parent.Children[i-1].ID); i-- {
		parent.Children[i], parent.Children[i-1] =
			parent.Children[i-1], parent.Children[i]
	}
}

func makeTree(n int) []Node {
	// make an array of n Nodes; set every node’s sID to -1 to denote
	// uninitialized node, since the default 0 already denotes root
	tree := make([]Node, n)
	for i := 0; i < n; i++ {
		tree[i].ID = -1
	}
	return tree
}

// Build returns the root of the tree compiled from the array of records given
// as input.  It returns an error if the input is invalid.
func Build(records []Record) (*Node, error) {
	n := len(records)
	// nil tree for nil list
	if n == 0 {
		return nil, nil
	}
	// sorting largely improves the BenchmarkShallowTree-8 case; the other two
	// cases (BenchmarkTwoTree-8, BenchmarkTenTree-8) seem fairly unaffected.
	sort.Slice(records, func(i, j int) bool {
		return records[i].ID < records[j].ID
	})
	if records[0].ID != 0 {
		return nil, errors.New("missing root")
	}

	tree := makeTree(n)
	// iterate over records, update node ID and report to parent
	for _, r := range records {
		// check for invalid and duplicate records
		err := validate(r, tree)
		if err != nil {
			return nil, err
		}

		// fix the placeholder -1 with the correct ID
		tree[r.ID].ID = r.ID

		// report to parent, if not root; root has no parent
		if r.ID != 0 {
			appendChild(&tree[r.Parent], &tree[r.ID])
		}
	}
	return &tree[0], nil
}

func chk(n *Node, m int) (err error) {
	if n.ID > m {
		return fmt.Errorf("z")
	} else if n.ID == m {
		return fmt.Errorf("y")
	} else {
		for i := 0; i < len(n.Children); i++ {
			err = chk(n.Children[i], m)
			if err != nil {
				return
			}
		}
		return
	}
}

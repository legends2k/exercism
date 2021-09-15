// Package tree constructs a tree from an array of nodes
package tree

import (
	"errors"
	"fmt"
)

type Record struct {
	ID, Parent int
}

type Node struct {
	ID       int
	Children []*Node
}

func validate(r Record, rec []*Node) error {
	if r.ID >= len(rec) {
		return errors.New("invalid node ID")
	} else if (r.ID != 0) && (r.ID <= r.Parent) {
		// if not root, confirm if parent ID < ID
		return errors.New("invalid parent ID")
	} else if (r.ID == 0) && (r.Parent != 0) {
		// make sure root has itself as parent
		return errors.New("invalid root parent")
	} else if (rec[r.ID] != nil) && (rec[r.ID].ID != -1) {
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

// Build returns the root of the tree compiled from the array of records given
// as input.  It returns an error if the input is invalid.
func Build(records []Record) (*Node, error) {
	// nil tree for nil list
	if len(records) == 0 {
		return nil, nil
	}
	n := len(records)
	var err error
	// make an array of Node pointers for all records in the input
	rec := make([]*Node, n)
	// iterate over records, creating nodes and reporting to the parent
	for _, r := range records {
		// check for invalid and duplicate records
		err := validate(r, rec)
		if err != nil {
			return nil, err
		}
		// if absent, create; if present fix the placeholder
		if rec[r.ID] == nil {
			rec[r.ID] = &Node{r.ID, nil}
		} else {
			// children already in place; just fix placeholder ID
			rec[r.ID].ID = r.ID
		}
		// report to parent, if not root; root has no parent
		if r.ID != 0 {
			// if parent absent, create a placeholder
			if rec[r.Parent] == nil {
				rec[r.Parent] = &Node{-1, nil}
			}
			appendChild(rec[r.Parent], rec[r.ID])
		}
	}
	if rec[0].ID != 0 {
		err = errors.New("missing root")
	}
	return rec[0], err
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

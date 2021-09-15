// Package tournament tallies the cores of teams
package tournament

import (
	"encoding/csv"
	"errors"
	"fmt"
	"io"
	"sort"
)

type score struct {
	played int
	won    int
	drawn  int
	lost   int
	points int
}

// io.Reader is an interface while strings.Reader is a struct (a concrete
// type).  Pointer to interfaces are almost never used, while pointers to
// concrete types are used to avoid copies and to modify the original
// argument. Refer https://stackoverflow.com/a/21283083/183120

// Tally tallies the scores between different teams and thorws error if it
// cannot.
func Tally(r io.Reader, w io.Writer) error {
	rd := csv.NewReader(r)
	rd.Comma = ';'
	rd.Comment = '#'
	rec, err := rd.ReadAll()
	if err != nil {
		return err
	}

	t := map[string]score{}
	for _, r := range rec {
		if len(r) != 3 {
			return errors.New("Invalid record: require 3 fields")
		}
		if r[0] == r[1] {
			return errors.New("Invalid record: same team for both opponents")
		}
		// can’t edit struct fields in a map directly; make copies
		// https://stackoverflow.com/q/17438253/183120
		teamA := t[r[0]]
		teamB := t[r[1]]
		teamA.played++
		teamB.played++
		switch r[2] {
		case "win":
			teamA.won++
			teamB.lost++
			teamA.points += 3
		case "loss":
			teamA.lost++
			teamB.won++
			teamB.points += 3
		case "draw":
			teamA.drawn++
			teamB.drawn++
			teamA.points++
			teamB.points++
		default:
			return errors.New("Invalid record: unrecognizable match result")
		}
		t[r[0]] = teamA
		t[r[1]] = teamB
	}

	copyResults(w, &t)
	return nil
}

func copyResults(w io.Writer, t *map[string]score) {
	teams := sortMapByPoints(t)
	fmt.Fprintf(w, "%-30s | %2s | %2s | %2s | %2s | %2s\n", "Team", "MP",
		"W", "D", "L", "P")
	for _, i := range teams {
		fmt.Fprintf(w, "%-30s | %2v | %2v | %2v | %2v | %2v\n", i, (*t)[i].played, (*t)[i].won,
			(*t)[i].drawn, (*t)[i].lost, (*t)[i].points)
	}
}

func sortMapByPoints(m *map[string]score) []string {
	teams := make([]string, 0, len(*m))
	for k := range *m {
		teams = append(teams, k)
	}
	// Sort with custom comparator function; use greater instead of less for
	// descending order
	sort.Slice(teams, func(i, j int) bool {
		teamA := teams[i]
		teamB := teams[j]
		if (*m)[teamA].points != (*m)[teamB].points {
			return (*m)[teamA].points > (*m)[teamB].points
		}
		// lexicographical sort for tied teams
		return teamA < teamB
	})
	return teams
}
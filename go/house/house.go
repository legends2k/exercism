// package house is to learn recursion in Go
package house

import (
	"errors"
	"strings"
)

const testVersion = 1

const insPoint = 11

var phrase = [...]string{"This is the house that Jack built.",
	"malt\nthat lay in",
	"rat\nthat ate",
	"cat\nthat killed",
	"dog\nthat worried",
	"cow with the crumpled horn\nthat tossed",
	"maiden all forlorn\nthat milked",
	"man all tattered and torn\nthat kissed",
	"priest all shaven and shorn\nthat married",
	"rooster that crowed in the morn\nthat woke",
	"farmer sowing his corn\nthat kept",
	"horse and the hound and the horn\nthat belonged to",
}

func Verse(i int) string {
	if i < 1 || i > len(phrase) {
		panic(errors.New("Invalid argument"))
	} else if i == 1 {
		return phrase[0]
	}
	s := Verse(i - 1)
	return s[:12] + phrase[i-1] + " the " + s[12:]
}

func Song() string {
	verse := make([]string, len(phrase))
	for i := 1; i <= len(phrase); i++ {
		verse[i-1] = Verse(i)
	}
	return strings.Join(verse, "\n\n")
}

// change package to main and uncomment to test manually
// func main() {
// i, _ := strconv.Atoi(os.Args[1])
// fmt.Println(Verse(i))
// }
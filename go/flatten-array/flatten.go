package flatten

import (
	"reflect"
)

// Refer: https://blog.golang.org/laws-of-reflection
// dfs assumes its input as a slice and walks it depth-first
func dfs(in interface{}, out *[]interface{}) {
	s := reflect.ValueOf(in)
	for i := 0; i < s.Len(); i++ {
		val := s.Index(i)
		if !val.IsNil() {
			// s.Index() gives a value of the struct reflect.Value and not the
			// empty interface, which is what we want, since the input is a
			// slice of empty interfaces, so doing s.Index().(int) won’t work
			v := val.Interface()
			switch reflect.TypeOf(v).Kind() {
			case reflect.Slice:
				dfs(v, out)
			case reflect.Int:
				*out = append(*out, v.(int))
			}
		}
	}
}
func Flatten(l interface{}) []interface{} {
	r := make([]interface{}, 0)
	dfs(l, &r)
	return r
}

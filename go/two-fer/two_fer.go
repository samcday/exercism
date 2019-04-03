// Package twofer is the stage where Sam is already sick of writing package 
// level documentation.
package twofer

// ShareWith is a function. You execute it with parameters and it returns a
// result. Incredible.
func ShareWith(name string) string {
	if name == "" {
		name = "you";
	}
	// Initially I used fmt.Sprintf since that looks nicer, but then I realized
	// it's nearly 6x slower in benchmarks and results in a lot of allocations.
	return "One for " + name + ", one for me.";
}

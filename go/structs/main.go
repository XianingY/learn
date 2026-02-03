package main

import "fmt"

type Person struct {
	Name string
	Age  int
}

func (p Person) Greet() string {
	return fmt.Sprintf("Hi, I'm %s", p.Name)
}

func (p *Person) HaveBirthday() {
	p.Age++
}

func main() {
	person := Person{Name: "Ada", Age: 30}
	fmt.Println(person.Greet())
	person.HaveBirthday()
	fmt.Printf("Age after birthday: %d\n", person.Age)
}

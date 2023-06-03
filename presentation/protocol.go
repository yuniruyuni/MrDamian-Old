package presentation

type Candidate struct {
	Kind  string `json:"kind"`
	Label string `json:"label"`
}

type Candidates []Candidate

type Node struct {
	ID       string   `json:"id"`
	Kind     string   `json:"kind"`
	Position Position `json:"position"`
	Data     NodeData `json:"data"`
}

type NodeData struct {
	Label   string       `json:"label"`
	Inputs  []InputPort  `json:"inputs"`
	Outputs []OutputPort `json:"outputs"`
}

type Position struct {
	X float64 `json:"x"`
	Y float64 `json:"y"`
}

type Edge struct {
	ID           string   `json:"id"`
	Label        string   `json:"label"`
	Source       string   `json:"source"`
	Target       string   `json:"target"`
	SourceHandle string   `json:"sourceHandle"`
	TargetHandle string   `json:"targetHandle"`
	Data         EdgeData `json:"data"`
}

type EdgeData struct {
	Assignment Assignment `json:"assignment"`
}

type (
	Argument     string
	PropertyName string
	Assignment   map[Argument]PropertyName
)

type PropertyNames []PropertyName

type InputPort struct {
	Parent        string        `json:"parent"`
	Name          string        `json:"name"`
	PropertyNames PropertyNames `json:"propertyNames"`
}

type OutputPort struct {
	Parent        string        `json:"parent"`
	Name          string        `json:"name"`
	PropertyNames PropertyNames `json:"propertyNames"`
}

type (
	Nodes []Node
	Edges []Edge
)

type Editor struct {
	Nodes Nodes `json:"nodes"`
	Edges Edges `json:"edges"`
}

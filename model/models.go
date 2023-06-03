package model

type Kind string

type (
	Argument      string
	PropertyName  string
	Assignment    map[Argument]PropertyName
	PropertyNames []PropertyName
)

type Pipeline struct {
	Kind    Kind
	ID      string
	Outputs OutputPorts
	Inputs  InputPorts
}

type Connection struct {
	ID         string
	Source     InputPortID
	Target     OutputPortID
	Assignment Assignment
}

type InputPortID struct {
	Parent string
	Name   string
}

type InputPort struct {
	ID            InputPortID
	PropertyNames PropertyNames
}

type InputPorts []InputPort

type OutputPortID struct {
	Parent string
	Name   string
}

type OutputPort struct {
	ID            OutputPortID
	PropertyNames PropertyNames
}
type OutputPorts []OutputPort

type Candidate struct {
	Kind  Kind
	Label string
}

// The event for pipeline updated.
const PipelineUpdated = "pipeline-updated"

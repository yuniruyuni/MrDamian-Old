package pipeline

import (
	"github.com/yuniruyuni/mrdamian/config"
	"github.com/yuniruyuni/mrdamian/model"
)

type Constructor struct {
	Kind  string
	Label string
	Gen   func(id string, c *config.Config) Component
}

type Component interface {
	ID() string
	Kind() string
	Label() string
	Inputs() model.InputPorts
	Outputs() model.OutputPorts

	Run() Process
}

type Process interface {
	Run(conn *Connection)
}

type PassiveProcess struct {
	Handler func(*Packet) []*Packet
}

func (p *PassiveProcess) Run(conn *Connection) {
	for {
		packet, err := conn.Receive()
		if err != nil {
			return
		}

		packets := p.Handler(packet)
		for _, packet := range packets {
			if err := conn.Send(packet); err != nil {
				// TODO: logging
				continue
			}
		}
	}
}

type ActiveProcess struct {
	Handler func() []*Packet
}

func (p *ActiveProcess) Run(conn *Connection) {
	for {
		packets := p.Handler()
		for _, packet := range packets {
			if err := conn.Send(packet); err != nil {
				// TODO: logging
				continue
			}
		}
	}
}

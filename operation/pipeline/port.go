package pipeline

import "github.com/yuniruyuni/mrdamian/model"

type (
	Sender   chan<- *Packet
	Receiver <-chan *Packet
)

type InputPort struct {
	BaseSender Sender
	Receiver   Receiver
}

type InputPorts map[string]InputPort

const PortBufferSize = 32

func NewInputPort() InputPort {
	ch := make(chan *Packet, PortBufferSize)
	return InputPort{
		BaseSender: ch,
		Receiver:   ch,
	}
}

func (p *InputPort) Accquire(dest string) OutputPort {
	return OutputPort{
		Dest:   dest,
		Sender: p.BaseSender,
	}
}

func (p *InputPort) Receive() (*Packet, error) {
	return <-p.Receiver, nil
}

type OutputPort struct {
	Dest   string
	Sender Sender
}

func (p *OutputPort) Send(message Message) {
	packet := Packet{
		Port:    p.Dest,
		Message: message,
	}
	p.Sender <- &packet
}

type OutputPorts map[string][]OutputPort

func NewOutputPorts() OutputPorts {
	return OutputPorts{}
}

func (p OutputPorts) Attach(src string, port OutputPort) {
	p[src] = append(p[src], port)
}

func (p OutputPorts) Send(packet *Packet) error {
	port, ok := p[packet.Port]
	if !ok {
		return model.ErrPortNotFound
	}

	for _, p := range port {
		p.Send(packet.Message)
	}

	return nil
}

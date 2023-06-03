package pipeline

type Connection struct {
	Input   InputPort
	Outputs OutputPorts
}

func NewConnection() *Connection {
	return &Connection{
		Input:   NewInputPort(),
		Outputs: NewOutputPorts(),
	}
}

func (c *Connection) Receive() (*Packet, error) {
	return c.Input.Receive()
}

func (c *Connection) Send(packet *Packet) error {
	return c.Outputs.Send(packet)
}

func (c *Connection) Connect(src, dst *Connection, srcPort, dstPort string) {
	src.Attach(srcPort, dst.Accquire(dstPort))
}

func (c *Connection) Attach(src string, port OutputPort) {
	c.Outputs.Attach(src, port)
}

func (c *Connection) Accquire(port string) OutputPort {
	return c.Input.Accquire(port)
}

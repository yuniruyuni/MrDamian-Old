package pipeline

type Packet struct {
	Port    string
	Message Message
}

type Packets []Packet

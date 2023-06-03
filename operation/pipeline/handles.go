package pipeline

type (
	Handle  chan<- *Packet
	Handles []Handle
)

func NewHandles() Handles {
	return Handles{}
}

func (hs Handles) Send(packet *Packet) {
	for _, h := range hs {
		h <- packet
	}
}

func (hs Handles) Close() {
	for _, h := range hs {
		close(h)
	}
}

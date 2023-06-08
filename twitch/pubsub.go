package twitch

import (
	"context"
	"encoding/json"

	"github.com/morikuni/failure"
	"github.com/nicklaw5/helix/v2"
	"golang.org/x/net/websocket"
)

const twitchWebsocketURL = "wss://eventsub.wss.twtich.tv/ws"
const originURL = "http://localhost:8787"

type ReconnectURL string

func (url ReconnectURL) String() string {
	return string(url)
}

type PubSubClient struct {
	socket *websocket.Conn
	Client *helix.Client

	SessionID    SessionID
	ReconnectURL ReconnectURL
}

func New(token string) (*PubSubClient, error) {
	return NewWithURL(twitchWebsocketURL, token)
}

func NewWithURL(url, token string) (*PubSubClient, error) {
	socket, err := websocket.Dial(url, "", originURL)
	if err != nil {
		// TODO: error handling.
		return nil, failure.Wrap(err)
	}

	client := &PubSubClient{
		socket: socket,
	}

	return client, nil
}

type MessageID string
type MessageType string
type MessageTimestamp string
type Metadata struct {
	MessageID        MessageID        `json:"message_id"`
	MessageType      MessageType      `json:"message_type"`
	MessageTimestamp MessageTimestamp `json:"message_timestamp"`
}

type Message struct {
	Metadata Metadata `json:"metadata"`
	Payload  any      `json:"payload"`
}

type SessionID string

func (id SessionID) String() string {
	return string(id)
}

type WelcomeMessage struct {
	MetaData Metadata
	Payload  struct {
		Session struct {
			ID                      SessionID `json:"id"`
			Status                  string
			KeepaliveTimeoutSeconds int
			ReconnectURL            ReconnectURL `json:"reconnect_url"`
			ConnectedAt             string
		}
	}
}

type Listener func(msg *Message) error
type Listeners map[MessageType]Listener

func (c *PubSubClient) OnWelcomMessage(raw *Message) error {
	msg, err := Refine[WelcomeMessage](raw)
	if err != nil {
		return failure.Wrap(err)
	}

	c.SessionID = msg.Payload.Session.ID
	c.ReconnectURL = msg.Payload.Session.ReconnectURL

	return nil
}

func (c *PubSubClient) Run(ctx context.Context) {
	ls := Listeners{
		"session_welcome": c.OnWelcomMessage,
	}

	for {
		select {
		case <-ctx.Done():
			return
		default:
			received := Message{}
			if err := websocket.JSON.Receive(c.socket, &received); err != nil {
				// TODO: write logging for errors from Receive
				// TODO: check shutdown or pure error
				continue
			}

			listener, ok := ls[received.Metadata.MessageType]
			if !ok {
				// TODO: write warn for unsupported mesasge.
				continue
			}
			if err := listener(&received); err != nil {
				// TODO: logging errors
				continue
			}
		}
	}
}

func Refine[T any](msg *Message) (T, error) {
	// TODO: find better way for unmarshaling arbitrary typed object.

	var detailed T
	data, err := json.Marshal(msg)
	if err != nil {
		return detailed, failure.Wrap(err)
	}

	if err := json.Unmarshal(data, &detailed); err != nil {
		return detailed, failure.Wrap(err)
	}
	return detailed, nil
}

func (c *PubSubClient) Close() {
	if c != nil {
		c.socket.Close()
	}
}

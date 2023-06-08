package twitch_test

import (
	"context"
	"encoding/json"
	"fmt"
	"io"
	"net/http/httptest"
	"testing"

	"github.com/yuniruyuni/mrdamian/twitch"
	"golang.org/x/net/websocket"
	"gotest.tools/v3/assert"
)

type Mode int

const (
	Send Mode = 0
	Recv Mode = 1
)

type Message map[string]any
type Protocol struct {
	mode    Mode
	message Message
}
type Protocols []Protocol

func startServer(t *testing.T, cancel context.CancelFunc, ps Protocols) *twitch.PubSubClient {
	t.Helper()

	server := httptest.NewServer(websocket.Handler(func(conn *websocket.Conn) {
		defer conn.Close()

		for _, m := range ps {
			m := m
			switch m.mode {
			case Send:
				{
					data, err := json.Marshal(&m.message)
					assert.NilError(t, err)

					_, err = conn.Write(data)
					assert.NilError(t, err)
				}
			case Recv:
				{
					var f []byte
					_, err := conn.Read(f)
					if err != io.EOF {
						assert.NilError(t, err)
					}

					received := Message{}
					err = json.Unmarshal(f, &received)
					assert.NilError(t, err)

					assert.DeepEqual(t, m.message, received)
				}
			}
		}

		cancel()
	}))
	addr := server.Listener.Addr().String()
	client, err := twitch.NewWithURL(
		fmt.Sprintf("ws://%s%s", addr, "/"),
		"dummy-token",
	)
	assert.NilError(t, err)
	t.Cleanup(server.Close)
	t.Cleanup(client.Close)

	return client
}

func TestNew(t *testing.T) {
	ctx, cancel := context.WithCancel(context.Background())
	client := startServer(t, cancel, Protocols{
		{
			Send,
			Message{
				"metadata": Message{
					"message_id":        "dummy-id",
					"message_type":      "session_welcome",
					"message_timestamp": "123456789",
				},
				"payload": Message{
					"session": Message{
						"id":            "dummy-session-id",
						"reconnect_url": "ws://example.com/",
					},
				},
			},
		},
	})
	client.Run(ctx)

	assert.DeepEqual(t, twitch.SessionID("dummy-session-id"), client.SessionID)
	assert.DeepEqual(t, twitch.ReconnectURL("ws://example.com/"), client.ReconnectURL)
}

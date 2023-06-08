package subscriber

import (
	"github.com/nicklaw5/helix/v2"
	"github.com/yuniruyuni/mrdamian/operation/pipeline"
)

type Process struct {
	pipeline.ActiveProcess

	// Client    *pubsub.PubSubClient
	Token     string
	ChannelID string
	BotID     string
}

func NewSubscriberProcess(p *Subscriber) *Process {
	client, err := helix.NewClient(&helix.Options{
		UserAccessToken: p.OAuth,
	})
	if err != nil {
		// TODO: errro handling.
		return nil
	}

	resp, err := client.GetUsers(
		&helix.UsersParams{
			Logins: []string{
				p.Channel,
				p.Bot,
			},
		})
	if err != nil {
		// TODO: error hanlding.
		return nil
	}
	if len(resp.Data.Users) != 1 {
		// TODO: error hanlding.
		return nil
	}

	// userId := resp.Data.Users[0].ID

	proc := &Process{
		// Client:    client,
		Token:     p.OAuth,
		ChannelID: resp.Data.Users[0].ID,
		BotID:     resp.Data.Users[1].ID,
	}
	proc.ActiveProcess.Handler = proc.Handler
	return proc
}

func (p *Process) Handler() []*pipeline.Packet {
	return nil
}

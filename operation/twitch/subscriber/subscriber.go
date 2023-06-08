package subscriber

import (
	"github.com/yuniruyuni/mrdamian/config"
	"github.com/yuniruyuni/mrdamian/model"
	"github.com/yuniruyuni/mrdamian/operation/pipeline"
)

type Subscriber struct {
	id      string
	OAuth   string
	Channel string
	Bot     string
}

func Constructor() pipeline.Constructor {
	return pipeline.Constructor{
		Kind:  "TwitchSubscriber",
		Label: "Twitch Subscriber",
		Gen: func(id string, c *config.Config) pipeline.Component {
			return NewSubscriber(id, c.Bot, c.Channel, c.Token)
		},
	}
}

func NewSubscriber(id string, bot string, channel string, oauth string) *Subscriber {
	return &Subscriber{
		id:      id,
		OAuth:   oauth,
		Channel: channel,
		Bot:     bot,
	}
}

func (p *Subscriber) ID() string {
	return p.id
}

func (p *Subscriber) Kind() string {
	return "TwitchSubscriber"
}

func (p *Subscriber) Label() string {
	return "Twitch Subscriber"
}

func (p *Subscriber) Inputs() model.InputPorts {
	return model.InputPorts{
		model.InputPort{
			ID: model.InputPortID{
				Parent: p.id,
				Name:   "message",
			},
			PropertyNames: model.PropertyNames{
				"from_broadcaster_user_login",
				"from_broadcaster_user_id",
				"viewers",
			},
		},
	}
}

func (p *Subscriber) Outputs() model.OutputPorts {
	return model.OutputPorts{}
}

func (p *Subscriber) Run() pipeline.Process {
	return NewSubscriberProcess(p)
}
